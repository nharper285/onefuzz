// Rust generated by Microsoft, code originally from:
//  https://github.com/google/clusterfuzz/blob/master/src/
//      python/lib/clusterfuzz/stacktraces/constants.py
//
// Original Copyright:
//
// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate lazy_static;

use regex::RegexSet;

mod generated;

pub fn get_stack_filter() -> &'static RegexSet {
    // NOTE: this is build upon first use, but we've verified these will compile
    // using unit tests
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(generated::STACK_FRAME_IGNORE_REGEXES)
            .expect("libclusterfuzz regex compile error");
    }
    &RE
}

#[cfg(test)]
mod tests {
    use super::get_stack_filter;

    #[test]
    fn test_stack_filter() {
        assert!(get_stack_filter().is_match("abort"));
        assert!(!get_stack_filter().is_match("ContosoSaysHi"));
        assert!(get_stack_filter().is_match("libc.so"));
    }
}
