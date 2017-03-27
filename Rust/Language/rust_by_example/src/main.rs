mod formatted_print;
mod primitives;
mod custom_types;
mod variable_bindings;
mod casting;
mod expressions;

fn main() {
    formatted_print::formatted_print();
    primitives::primitives();
    custom_types::custom_types();
    variable_bindings::variable_bindings();
    casting::casting();
    expressions::expressions();
}
