use darling::{error::Accumulator, Result};

pub(crate) trait AccumulatorExt {
    fn push_result<T>(&mut self, res: &Result<T>);

    fn finnish_with_result<T>(self, res: Result<T>) -> Result<T>;
}

impl AccumulatorExt for Accumulator {
    fn push_result<T>(&mut self, res: &Result<T>) {
        match res {
            Ok(_) => (),
            Err(err) => self.push(err.clone()),
        }
    }

    fn finnish_with_result<T>(mut self, res: Result<T>) -> Result<T> {
        match res {
            Ok(success) => self.finish_with(success),
            Err(err) => {
                self.push(err);
                let final_err = self.finish().expect_err("known to be an Err(_)");
                Err(final_err)
            }
        }
    }
}
