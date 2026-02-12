/**************************************
- Author: Clement Poncelet
- Desc: Trait Constraint, API for intensional or extensional constraints
- TODO: Refine what should be delegate to extensional implementation
- Optimization:
    - HashSet for constraints scopes and extensional tables
***************************************/

/**************************************
            Trait Constraint
***************************************/
use std::cmp::Ordering::Equal;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use crate::csp::domain::setdom::{CartesianWalker};
use crate::csp::domain::domain::{Domain, OrdT};
use crate::csp::truth::Truth;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::{make_assignment, vv, VValue};
use crate::csp::csp::exists_extension;

pub trait Constraint<T:OrdT> : Debug {

    //Trait : Methods to implement ---

    //Display trait implementation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    //(operator semantics)
    fn apply(&self, asn: &Vec<VValue<T>>) -> bool;
    //Scope, return the variable(s) implied by the constraint
    fn scp(&self) -> &[Rc<ExVar<T>>];

    //Trait : Methods to implement --- END

    //Constraint's label
    //Generated with c_ and the variables' label in its scope
    fn label(&self) -> String { format!("c_{}", self.scp().iter().map(|v| v.label())
        .cloned()
        .collect::<Vec<_>>()
        .join(""))}

    // Is the v-value <x,v> accepted by the constraint:
    // - Unknown, if out of scope
    // - True, if the constraint can be satisfied (delegate to is_support_asn)
    // - False otherwise
    fn is_allowed(&self, vvalue: &VValue<T>) -> Truth {
        if self.scp().iter().any( |v| v.label() == &vvalue.label) {
            Truth::from(self.is_support_asn(&vec![vvalue.clone()], true))
        } else {
            Truth::Unknown
        }
    }

    // Applies assignment with a scope check:
    // - Unknown, if out of scope
    // - True, False, output of applying the constraint
    fn check_assignment(&self, asn: &Vec<VValue<T>>) -> Truth {
        if asn.len() < self.scp().len() { Truth::Unknown }
        else { Truth::from(self.apply(asn)) }
    }

    // From a v-value (x, a), returns:
    //- Truth::True if the v-value is valid for the corresponding variable's constraint
    //- Truth::False if the value is not a possible value for x
    //- Truth::Unknown if x is not in scp(c)
    fn is_valid(&self, vvalue: &VValue<T>) -> Truth {
        match self.scp().iter().find( |v| v.label() == &vvalue.label) {
            Some(v) => {
                if v.valid_values().contains(&vvalue.value) {
                    Truth::True
                } else {
                    Truth::False
                }
            },
            None => Truth::Unknown, // variable not in scope ⇒ constraint unaffected
        }
    }

    // From a v-value (x, a), returns:
    //- Truth::True if the v-value is a support for the constraint
    //- Truth::Unknown if x is not in scp(c)
    //- Truth::False otherwise
    fn is_support(&self, vvalue: &VValue<T>) -> Truth {
        match self.is_valid(vvalue) {
            Truth::True => self.is_allowed(vvalue),
            other => other,
        }
    }

    // From a v-value (x, a), returns:
    //- Truth::True if the v-value is a conflict for the constraint
    //- Truth::Unknown if x is not in scp(c)
    //- Truth::False otherwise
    fn is_conflicts(&self, vvalue: &VValue<T>) -> Truth {
        match self.is_support(vvalue) {
            Truth::True => Truth::False,
            Truth::False => Truth::True,
            Truth::Unknown => Truth::Unknown,
        }
    }

    // From an assignment (vec of v-values (x_i, a_i)), returns:
    //- true if the v-values covers all the constraint's scope,
    //- false otherwise
    fn is_covered(&self, asn: &Vec<VValue<T>>) -> bool {
        self.scp().iter().all(|var| {
            asn.iter().any(|vv| vv.label == *var.label())
        })
    }

    // From an assignment uses !cartesian product!, and returns:
    //- True if asn is a support for the constraint,
    //- False otherwise
    fn is_support_asn_rel(&self, asn: &Vec<VValue<T>>) -> Truth {
        // if assignment contradicts domain → invalid
        for vv in asn {
            if self.is_valid(vv) == Truth::False {
                return Truth::False;
            }
        }

        // if assignment fully instantiates the constraint
        if self.is_covered(asn) {
            return self.check_assignment(asn);
        }

        // otherwise: ∃ extension that satisfies the constraint
        for tuple in self.rel() {
            if asn.iter().all(|vv| tuple.contains(vv)) {
                return Truth::True;
            }
        }
        Truth::False
    }

    // From an assignment uses assignment's extension
    // skip_valid is for mimicking is_allowed
    // Returns:
    // - True if asn is a support for the constraint,
    // - False otherwise
    fn is_support_asn(&self, asn: &Vec<VValue<T>>, skip_valid: bool) -> Truth {
        if !skip_valid {
            // if assignment contradicts domain → invalid
            for vv in asn {
                if self.is_valid(vv) == Truth::False {
                    return Truth::False;
                }
            }

            // if assignment fully instantiates the constraint
            if self.is_covered(asn) {
                return self.check_assignment(asn);
            }
        }

        // otherwise: ∃ extension that satisfies the constraint
        // get unnassigned vars' dom
        let assigned_labels: std::collections::HashSet<_> =
            asn.iter().map(|v| &v.label).collect();

        let var_to_extends: Vec<_> = self.scp()
            .iter()
            .filter(|v| !assigned_labels.contains(v.label()))
            .cloned()
            .collect();

        if exists_extension(asn, &var_to_extends, |full | {
            self.check_assignment(full) == Truth::True }) {
            Truth::True
        } else {
            Truth::False
        }
    }

    //[To check]
    fn strict_support(&self, vvalue: &VValue<T>) -> Truth {
        // First: must be valid for the variable
        if self.is_valid(vvalue) != Truth::True {
            return Truth::False;
        }

        // Then: must appear in at least one fully valid tuple
        for tuple in self.rel().iter() {
            if tuple.iter().any(|vv| vv == vvalue) {
                let ok = tuple.iter().all(|vv| {
                    self.is_valid(vv) == Truth::True
                });
                if ok {
                    return Truth::True;
                }
            }
        }
        Truth::False
    }

    //Return
    // - true if the constraint is always satisfied
    // - false otherwise
    fn is_entailed(&self) -> bool {
        if self.rel().iter().any(|t| !self.check_assignment(t).to_bool().unwrap())
        { false }  else { true }
    }

    //Return
    // - true if the constraint has no support
    // - false otherwise
    fn is_disentailed(&self) -> bool {
        if self.rel().iter().any(|t| self.check_assignment(t).to_bool().unwrap())
        { false }  else { true }
    }

    // 1.0 - self.tightness()
    fn looseness(&self) -> f64 {
        let tot = self.size();
        let mut allowed = 0;

        //extend to cartesian product for any cardinality
        for x in self.scp()[0].valid_values() {
            for y in self.scp()[1].valid_values() {
                if self.apply(&vec![vv(self.scp()[0].label().clone(), x.clone()),
                                    vv(self.scp()[1].label().clone(), y.clone())]) {
                    allowed += 1;
                }
            }
        }
        allowed as f64 / tot as f64
    }

    //Constraint tightness
    //  0.0 → very loose
    //  1.0 → impossible constrain
    fn tightness(&self) -> f64 {
        let tot = self.size();
        let mut forbidden = 0;

        //extend to cartesian product for any cardinality
        for x in self.scp()[0].valid_values() {
            for y in self.scp()[1].valid_values() {
                if !self.apply(&vec![vv(self.scp()[0].label().clone(), x.clone()),
                                     vv(self.scp()[1].label().clone(), y.clone())]) {
                    forbidden += 1;
                }
            }
        }
        forbidden as f64 / tot as f64
    }

    //Return all allowed assignments (!Cartesian product!)
    fn rel(&self) -> Vec<Vec<VValue<T>>> {
        let mut ret = Vec::new();
        let scope = self.scp();
        let mut walker = self.cartesian_product();

        while let Some(values) = walker.next() {
            let assignment = make_assignment(scope, values);

            if self.check_assignment(&assignment) == Truth::True {
                ret.push(assignment);
            }
        }
        ret
    }

    //---------------------------------------------------
    //                      Utilities
    //---------------------------------------------------
    // get corresponding operand from vvalue
    fn match_var(&self, vvalue: &VValue<T>) -> Option<&Rc<ExVar<T>>> {
        self.scp()
            .iter()
            .find(|v| v.label().cmp(&vvalue.label) == Equal)
    }

    fn other_var(&self, vvalue: &VValue<T>) -> Option<&Rc<ExVar<T>>> {
        if self.scp()[0].label() == &vvalue.label {
            Some(&self.scp()[1])
        } else if self.scp()[1].label() == &vvalue.label {
            Some(&self.scp()[0])
        } else {
            None
        }
    }

    fn value_of<'a>(&self, var: &ExVar<T>, asn: &'a [VValue<T>]) -> Option<&'a T> {
        asn.iter()
            .find(|vv| vv.label == *var.label())
            .map(|vv| &vv.value)
    }

    //Cartesian product (size)
    fn size(&self) -> usize {
        self.scp()
            .iter()
            .map(|v| v.valid_values().len())
            .product()
    }

    fn cartesian_product(&self) -> CartesianWalker<T> {
        let doms: Vec<Vec<T>> = self.scp()
            .iter()
            .map(|v| v.dom().active_values())
            .collect();
        CartesianWalker::new(doms)
    }
}

impl<T:OrdT> fmt::Display for dyn Constraint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Constraint::fmt(self, f)
    }
}