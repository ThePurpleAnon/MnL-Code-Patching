.nds


HEAP0_ARENA_HI_ORIG equ 0x23E0000
OVERLAY141_SPACE equ 0x10000
HEAP0_SHRUNK_BY equ OVERLAY141_SPACE
HEAP0_ARENA_HI equ HEAP0_ARENA_HI_ORIG - HEAP0_SHRUNK_BY
OVERLAY141_ADDRESS equ HEAP0_ARENA_HI


.open "bis-data/arm9.dec.bin", 0x02004000
.org 0x020058BC
getScriptVar:
.org 0x02005A78
setScriptVar:
.org 0x02019CF0
  sub r3, lr, 0x3E0000 - HEAP0_SHRUNK_BY
.org 0x02019DE8
  .word HEAP0_ARENA_HI - 12
.org 0x02019EAC
heapAlloc:
.org 0x0201A058
heapFree:
.org 0x02039798
  .word HEAP0_ARENA_HI
.org 0x0203D8E4
FS_InitFile:
.org 0x0203DBC4
FS_OpenFile:
.org 0x0203DC0C
FS_CloseFile:
.org 0x0203DD60
FS_ReadFile:
.org 0x0203E640
FS_LoadOverlay:

.ifdef F_CUSTOM_ITEM_TYPES
.org 0x020185B4
  b   get_custom_item_name_index
.org 0x02005CA4
  b   get_item_name_string
.org 0x0201883C
  b   GetNonBadgeItemAmountInjection
.org 0x020188B8
  b   AddItemsInjection
.endif

.org 0x02055F5C
.region 0x74
LoadOverlay141:
  mov r0, 0  ; MIProcessor target = MI_PROCESSOR_ARM9
  mov r1, 141  ; FSOverlayID id
  bl  FS_LoadOverlay

  bl  custom_init

  mov r1, 0
  b   PostLoadOverlay141Injection
.endregion
.close


.open "bis-data/overlay.dec/overlay_0001.dec.bin", 0x02069820
.org 0x02069980
  b   LoadOverlay141
PostLoadOverlay141Injection:
.close


.open "bis-data/overlay.dec/overlay_0013.dec.bin", 0x0208A240
.ifdef F_IMPOSSIBLE_MODE
.org 0x0208A36C
  b ImpossibleMode
.endif
.close


.open "bis-data/overlay.dec/overlay_0122.dec.bin", 0x02069820
.ifdef F_CUSTOM_ITEM_TYPES
.org 0x0206D668
  b   GetItemIconTail
.org 0x0206D6EC
  b   get_custom_item_description_index
.org 0x0206D5A0
  b   ItemListGetItemAmountInjection
PostItemListGetItemAmountInjection:
.endif
.close


.open "bis-data/overlay.dec/overlay_0124.dec.bin", 0x02076080
.ifdef F_MIXED_SHOP
; Makes it so that item descriptions are always fetched by their ID, irrespective of the shop type.
.org 0x02098BA4
  nop
; Makes consumable item shops display 5 digits of prices instead of 4, to make them like the others.
.org 0x020A1654
  .hword 5

; Makes it so that the max number of a given item you can buy depends on its ID and not just the shop type.
.org 0x02098E04
  bl  get_shop_max_items
.org 0x0209901C
  b   get_shop_max_selected_items

; Makes the grayed out badge icons loaded and used in consumable item shops.
.org 0x0208DDA0
  b   0x0208DDB8
.org 0x02097184
  movlt r1, 0x03
  movlt r0, 0x1F
; TODO: ???
; .org 0x0209717c
;   movge r1, 0x02
;   movge r0, 0x04
.endif
.close


;.open "bis-data/overlay.dec/overlay_0132.dec.bin", 0x02076080
;.org 0x02076714
;FUN_overlay_d_132__02076714:
;.org 0x02076248
;FUN_overlay_d_132__02076248:
;.org 0x02076354
;FUN_overlay_d_132__02076354:
;.org 0x020763B4
;FUN_overlay_d_132__020763b4:
;.org 0x020768a8
;  b   THook
;PostTHook:
;.org 0x02076F08
;  ;cmp r0, 0
;.org 0x02076968
;  ;mov r0, 0
;.close


.open "bis-data/overlay.dec/overlay_0138.dec.bin", 0x02063B00
.ifdef F_ANTI_PIRACY_PATCH
; Anti-anti-piracy.
.org 0x0206404C
  mov r11, 0x8000
.org 0x020641F4
  .word 0x1A017078
.endif
.close


.open "bis-data/overlay.dec/overlay_0139.dec.bin", 0x020647C0
.ifdef F_ANTI_PIRACY_PATCH
; Anti-anti-piracy.
.org 0x02064814
  mov r0, r0
.endif
.close


.create "bis-data/overlay.dec/overlay_0141.dec.bin", OVERLAY141_ADDRESS
.area OVERLAY141_SPACE
;THook:
;  cmp r0, 254
;  beq 0x02076968
;
;  push {r0-r2,lr}
;
;  mov r0, r4
;  bl  t_hook
;
;  cmp r0, 1
;  pop {r0-r2,lr}
;  popeq {r3, r4, r5, r6, r7, r8, r9, r10, r11, pc}
;  cmp r0, 0x67
;  b   PostTHook

.ifdef F_CUSTOM_ITEM_TYPES
GetItemIconTail:
  bxeq  lr
  b   get_custom_item_icon
ItemListGetItemAmountInjection:
  cmp   r1, 0x2000
  cmpne r1, 0x4000
  movne r0, r4
  bne   get_custom_item_amount
  ldr r3, [r0]
  b   PostItemListGetItemAmountInjection
GetNonBadgeItemAmountInjection:
  ldr r2, =0x0FFF
  and r0, r0, r2
  b   get_custom_item_amount
  .pool
AddItemsInjection:
  bl  add_custom_items
  pop {r3, pc}
.endif

; most of the following code is just an adaptation of skelux's AR code
.ifdef F_IMPOSSIBLE_MODE
ImpossibleMode:
  mov r3, r3, lsl #0x9
  ldr r12, =impossible_mode_address_0
  cmp r4, r12
  beq ImpossibleModeExit
  ldr r12, =impossible_mode_address_1
  cmp r4, r12
  beq ImpossibleModeExit
  ldr r12, =impossible_mode_address_2
  cmp r4, r12
  beq ImpossibleModeExit
  mov r3, r3, lsl #0x1
impossible_mode_address_0:
  .word 0210a470
impossible_mode_address_1:
  .word 0210a634
impossible_mode_address_2:
  .word 0210a7F8
ImpossibleModeExit:
  b 0x0208A370
.endif

.importobj "rust/target/armv5te-none-eabi/" + PROFILE + "/bis"
.endarea
.close


.open "bis-data/y9.bin", 0
.org 141 * 32
  .word 141  ; Overlay ID
  .word OVERLAY141_ADDRESS  ; RAM address
  .word filesize("bis-data/overlay.dec/overlay_0141.dec.bin")  ; RAM size
  .word 0  ; BSS size
  .word 0  ; Static initializer start address
  .word 0  ; Static initializer end address
  .word 141  ; File ID
  .word 0x01 << 24  ; Flags = compressed
.close
