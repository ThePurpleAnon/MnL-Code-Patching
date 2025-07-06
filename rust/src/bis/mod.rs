pub mod custom;

unsafe extern "C" {
    pub type clGlobalEvent;
    pub type ScriptState;
    pub type clMenuItemListBase;
    pub type clShop;

    pub fn getScriptVar(
        var: u16,
        global_event: *const clGlobalEvent,
        state: *const ScriptState,
    ) -> u32;
    pub fn setScriptVar(
        var: u16,
        value: u32,
        global_event: *const clGlobalEvent,
        state: *const ScriptState,
    );
}

pub const ITEM_NAME_MESSAGES: *mut *mut usize = 0x020564BC as *mut *mut usize;
