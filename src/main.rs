enum VariableType {
    Integer,
    //TODO: Implementation for real number looks complicated. Temporarily used Double 
    Double,
}

struct Variable {
    kind: VariableType,
    value: Value,
}

impl Variable {
    fn new_bool() -> Self {
        Variable{kind: VariableType::Integer, value: Value::Atomic(Atomic::Interval(Interval{lower: 0, upper: 1}))}
    }
    fn new_int() -> Self {
        Variable{kind: VariableType::Integer, value: Value::Atomic(Atomic::Interval(Interval{lower: 0, upper: 1}))}
    }
    fn new_double() -> Self {
        Variable{kind: VariableType::Integer, value: Value::Atomic(Atomic::Interval(Interval{lower: 0, upper: 1}))}
    }
}

struct Interval {
    lower: i64,
    upper: i64,
}

enum Atomic {
    // Closed interval of natural numbers
    Interval(Interval),
    // Concrete value
    Constant(i64),
    // Regular interval
    Chres(Chres),
}

// Defines the set of possible values ​​of a variable
enum Value {
    Atomic(Atomic),
    Composite(Vec<Atomic>),
}

impl Value {
    // fn intersection(&self, another: Self) {
    // }
    // fn disjunctive_union(&self, another: Self){
    // }
    
    fn add(){

    } 

    fn sub(){

    } 

    fn mul(){

    } 

    fn div(){

    }

    fn _mod(){

    }
}

enum ExecutionAbility {
    Always,
    Never,
    Conditionally, //TODO
}

struct Projection {
    // condition: ,

} 

enum Operator {
    Add,
    Mull,
}

// Chains of recurrences 
struct Chres {
    init: i64,
    operator: Operator,
    // In general, function defined over the natural numbers N. 
    // In the current implementation, the constant. In other words f(i) = constant
    stride: i64, 
}

impl Chres {
    fn add_c(mut self, constant: i64) {
        match self.operator {
            Add => {
                self.init += constant;
            },
            Mull => {
                panic!("forbidden operation");
            }
        }
    }
    fn mul_c(mut self, constant: i64) {
        match self.operator {
            Add => {
                self.init *= constant;
                self.stride *= constant;
            },
            Mull => {
                self.init *= constant;
            }
        }
    }
    fn is_pure_sume() {

    }
    fn is_pure_product() {

    }
    fn is_basic_recurrence() {

    }
    fn lenght() {

    }
    fn cost_index() {
        
    }
    // CR call simple when stride is constant
    fn is_simple() -> bool {
        true
    }
    fn lower_bound() {

    }
    fn upper_bound() {

    }

    // Checks if a value belongs to regular interval
    fn is_contain(&self, value: i64) -> bool {
        (value-self.init)/self.stride == 0
    }
}

struct Lattices {
    
}

// var a,b;  a = [min; max] b = [min; max]
// >> a;      
// 
// if(a%2 == 0) {                                             
//           lb = lower_that_satisfy(a, "/2 == 0");           
//           ub = upper_that_satisfy(a, "/2 == 0");           
//           a = intersection(a, (min;+;2), (lb, ub))  |      
//  b += 6;                                            |      
// }                                                 _/       
// 
// if(a%3 == 0) {                                             
//           lb = lower_that_satisfy(a, "/2 == 0");           
//           ub = upper_that_satisfy(a, "/2 == 0");           
//           a = intersection(a, (min;+;3), (lb, ub))  |      
//  b*= 12;                                            |      
// }                                                 _/       
// 
// if(b+5>a) {    
//          ????????????????????????
//  << a; 
// }

// Range propagation  
// 
// var a,b; [min; max] [min; max]
// >> a;      
// 
// if(a/2 == 0) {                                                 
//  b += 6;                            
// }                       
// 
// if(a/3 == 0) {                                             
//  b*= 12;                                            
// }                              
// 
// if(b+5>a) {    
//  << a; 
// }

// if(a == b) {
//  c = a - b;
// }


fn main() {
    println!("Hello, world!");
}
