/// ## [PATCH FLAG](https://github.com/vuejs/core/blob/main/packages/shared/src/patchFlags.ts)
#[derive(Debug)]
pub struct PatchFlag;

impl PatchFlag {
    pub const BAIL: isize = -2;
    pub const CLASS: isize = 1 << 1;
    pub const DYNAMIC_SLOTS: isize = 1 << 10;
    pub const FULL_PROPS: isize = 1 << 4;
    pub const HOISTED: isize = -1;
    pub const HYDRATE_EVENTS: isize = 1 << 5;
    pub const KEYED_FRAGMENT: isize = 1 << 7;
    pub const NEED_PATCH: isize = 1 << 9;
    pub const PROPS: isize = 1 << 3;
    pub const STABLE_FRAGMENT: isize = 1 << 6;
    pub const STYLE: isize = 1 << 2;
    pub const TEXT: isize = 1;
    pub const UN_KEYED_FRAGMENT: isize = 1 << 8;

    pub fn is_non_prop(flag: &isize) -> bool {
        // - 0b1_1_1_0 => Class_Style_Props_0
        flag & 0b1110 == 0
    }
}
