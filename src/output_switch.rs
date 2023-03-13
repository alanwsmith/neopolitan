use crate::output::output;
use crate::output_dev::output_dev;
use crate::page::Page;

pub fn output_switch(source: Page) -> String {
    // return output_dev(source);
    return output(source);
}
