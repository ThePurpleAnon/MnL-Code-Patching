#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![no_std]
#![no_main]
#![feature(ptr_as_ref_unchecked)]
#![feature(sync_unsafe_cell)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

extern crate alloc;

use alloc::{ffi::CString, format, vec::Vec};
use core::{cell::SyncUnsafeCell, ffi::c_char, ptr};

use mnl_code_patches::{
    bis::{
        ITEM_NAME_MESSAGES, clMenuItemListBase, clShop, custom::VarItemMetadata, getScriptVar,
        setScriptVar,
    },
    nds::{nitro::fs::file, nocash::print_paramln},
};

// static DB: SyncUnsafeCell<Vec<u8>> = SyncUnsafeCell::new(Vec::new());
static VAR_ITEM_METADATA: SyncUnsafeCell<Vec<VarItemMetadata>> = SyncUnsafeCell::new(Vec::new());

static ITEM_NAME_INDEX: SyncUnsafeCell<u16> = SyncUnsafeCell::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn custom_init() {
    print_paramln(c"Loading custom code...");

    // let mut db_file = file::open(c"/DB/DB.dat").unwrap();
    // print_paramln(c"db_file loaded!");
    // let mut db = vec![0u8; db_file.length() as usize];
    // db_file.read(&mut db[..]).unwrap();
    // unsafe { DB.get().as_mut_unchecked() }.append(&mut db);

    if let Ok(mut var_item_metadata_file) = file::open(c"/Custom/DB_VItm.dat") {
        let var_item_metadata = unsafe { VAR_ITEM_METADATA.get().as_mut_unchecked() };
        let length = (var_item_metadata_file.length() as usize) / size_of::<VarItemMetadata>();
        var_item_metadata.reserve_exact(length);
        unsafe {
            var_item_metadata_file
                .read_to_address(
                    var_item_metadata.as_mut_ptr().cast(),
                    (length * size_of::<VarItemMetadata>()).try_into().unwrap(),
                )
                .unwrap();
            var_item_metadata.set_len(length);
        }
    }
    /*println!("{:?}", unsafe {
        VAR_ITEM_METADATA.get().as_ref_unchecked()
    });*/

    print_paramln(c"Custom code loaded!");
}

/*unsafe extern "C" {
    fn FUN_overlay_d_132__02076714();
    fn FUN_overlay_d_132__02076248(msg: u32);
    fn FUN_overlay_d_132__02076354(param_1: u32) -> usize;
    fn FUN_overlay_d_132__020763b4(param_1: u32) -> usize;
}

#[unsafe(no_mangle)]
pub extern "C" fn t_hook(param_1: *mut u8) -> bool {
    let state = unsafe { *param_1.add(0x23) };
    println!("t_hook(state = {})!", state);
    match state {
        0 => {
            print_paramln(c"param_1 is 0!");
            unsafe {
                *(0x02056504 as *mut u32) |= 1;
                *(0x01FF93C4 as *mut u8) = 1;
                FUN_overlay_d_132__02076248(11);
                *(*param_1.add(0x0C).cast::<*mut usize>()) = FUN_overlay_d_132__02076354(0x10);
                *param_1.add(0x23) = 251;
            }
            print_paramln(c"param_1 is 0 end!");
            return true;
        }
        251 | 253 => {
            unsafe {
                if *(*(*param_1.add(0x0C).cast::<*mut *mut i8>())).add(0x21) == 1 {
                    *param_1.add(0x23) += 1;
                }
            }
            return true;
        }
        252 => {
            if unsafe { *(0x0205AFAA as *const u16) } & 0x0401 != 0 {
                unsafe {
                    //FUN_overlay_d_132__02076248(6);
                    *(*param_1.add(0x0C).cast::<*mut usize>()) = FUN_overlay_d_132__020763b4(0x10);
                    *param_1.add(0x23) += 1;
                }
            }
            return true;
        }
        255 => {
            unsafe {
                FUN_overlay_d_132__02076714();
                *param_1.add(0x23) = 1;
            }
            return false;
        }
        _ => {}
    }
    print_paramln(c"t_hook() end!");
    false
}*/

fn get_max_items_by_id(item_id: u32) -> i16 {
    let item_index = item_id & 0x0FFF;
    match item_id & 0xF000 {
        0x2000 => 99,
        0x3000 => 1,
        0x4000 => 9,
        0x5000 => {
            if (item_index as usize) < unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() }.len() {
                (unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() })[item_index as usize]
                    .max_amount()
                    .try_into()
                    .unwrap()
            } else {
                0
            }
        }
        _ => 99,
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn get_shop_max_items(
    _shop: *mut clShop,
    shop_type: i8,
    _: usize,
    item_id: u32,
) -> i16 {
    if shop_type == 3 {
        0x00FF
    } else {
        get_max_items_by_id(item_id)
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn get_shop_max_selected_items(shop: *mut clShop) -> i16 {
    let selected_item_id = unsafe {
        (*(*shop.cast::<*const unsafe extern "C" fn(*mut clShop) -> u32>()).byte_add(0x44))(shop)
    };
    let shop_type = unsafe { *shop.cast::<i8>().byte_add(0x7) };
    get_shop_max_items(shop, shop_type, 0, selected_item_id)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_custom_item_name_index(
    _item_id: u32,
    _: usize,
    item_type: u16,
    item_index: u16,
) -> *const u16 {
    match item_type {
        0x5000
            if usize::from(item_index)
                < unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() }.len() =>
        {
            unsafe {
                *ITEM_NAME_INDEX.get() =
                    VAR_ITEM_METADATA.get().as_ref_unchecked()[usize::from(item_index)].name_id;
            }
            ITEM_NAME_INDEX.get()
        }
        _ => {
            unsafe {
                *ITEM_NAME_INDEX.get() = 0xF000 | item_index;
            }
            ITEM_NAME_INDEX.get()
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn get_item_name_string(item_type: u16, name_index: u16) -> *const c_char {
    match item_type {
        0..=3 => unsafe {
            let messages = *ITEM_NAME_MESSAGES.add(item_type.into());
            messages
                .cast::<c_char>()
                .add(*messages.add(name_index.into()))
        },
        4 if name_index & 0xF000 != 0xF000 => {
            let mut bytes: Vec<u8> = format!("Var Item NI:{name_index:04X}").into();
            bytes.extend(b"\xff\x0a");
            unsafe { CString::from_vec_unchecked(bytes) }.into_raw()
        }
        _ => {
            let mut bytes: Vec<u8> = format!(
                "[UNK ITEM {:04X}!]",
                ((item_type + 1) << 12) | (name_index & 0x0FFF),
            )
            .into();
            bytes.extend(b"\xff\x0a");
            unsafe { CString::from_vec_unchecked(bytes) }.into_raw()
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn get_custom_item_icon(
    _item_list: *mut clMenuItemListBase,
    _item_id: u32,
    item_index: u16,
    item_type: u16,
) -> u8 {
    match item_type {
        0x5000
            if usize::from(item_index)
                < unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() }.len() =>
        {
            (unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() })[usize::from(item_index)]
                .sprite_id1
        }
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn get_custom_item_description_index(
    _item_list: *mut clMenuItemListBase,
    _item_id: u32,
    item_index: u16,
    item_type: u16,
) -> u16 {
    match item_type {
        0x5000
            if usize::from(item_index)
                < unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() }.len() =>
        {
            (unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() })[usize::from(item_index)]
                .description_id
        }
        _ => 0,
    }
}
// TODO: get_item_description_string(?)
// TODO: get_custom_item_price
#[unsafe(no_mangle)]
pub extern "C" fn get_custom_item_amount(item_index: u16, item_type: u16) -> u32 {
    match item_type {
        0x5000
            if usize::from(item_index)
                < unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() }.len() =>
        {
            let metadata =
                &(unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() })[usize::from(item_index)];
            let raw_amount = unsafe { getScriptVar(metadata.variable, ptr::null(), ptr::null()) };
            if metadata.amount_inverted() {
                u32::from(metadata.max_amount()) - raw_amount
            } else {
                raw_amount
            }
        }
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn add_custom_items(
    _item_id: u32,
    amount: i32,
    item_index: u16,
    item_type: u16,
) -> i32 {
    match item_type {
        0x5000
            if usize::from(item_index)
                < unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() }.len() =>
        {
            let metadata =
                &(unsafe { VAR_ITEM_METADATA.get().as_ref_unchecked() })[usize::from(item_index)];
            for (i, variable) in [metadata.secondary_variable, metadata.variable]
                .into_iter()
                .enumerate()
            {
                if i == 0 && variable == 0 {
                    continue;
                }
                let old_amount: i32 = unsafe { getScriptVar(variable, ptr::null(), ptr::null()) }
                    .try_into()
                    .unwrap();
                let amount = if metadata.amount_inverted() {
                    -amount
                } else {
                    amount
                };
                let new_amount = (old_amount + amount).clamp(0, metadata.max_amount().into());
                unsafe {
                    setScriptVar(
                        variable,
                        new_amount.try_into().unwrap(),
                        ptr::null(),
                        ptr::null(),
                    );
                }
                if i >= 1 {
                    return new_amount - old_amount;
                }
            }
            unreachable!()
        }
        _ => 0,
    }
}
