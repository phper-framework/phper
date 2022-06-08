// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use std::mem::{replace, take};

pub fn replace_and_set<T: Default>(t: &mut T, f: impl FnOnce(T) -> T) {
    let x = f(take(t));
    let _ = replace(t, x);
}

pub fn replace_and_get<T: Default, R>(t: &mut T, f: impl FnOnce(T) -> R) -> R {
    f(take(t))
}
