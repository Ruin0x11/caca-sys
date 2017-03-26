extern crate libc;

use std::os::raw::c_int;
use libc::{size_t, ssize_t, c_char, c_uint, c_float, c_void};

// Colors
pub const CACA_BLACK: c_uint =        0x00;
pub const CACA_BLUE: c_uint =         0x01;
pub const CACA_GREEN: c_uint =        0x02;
pub const CACA_CYAN: c_uint =         0x03;
pub const CACA_RED: c_uint =          0x04;
pub const CACA_MAGENTA: c_uint =      0x05;
pub const CACA_BROWN: c_uint =        0x06;
pub const CACA_LIGHTGRAY: c_uint =    0x07;
pub const CACA_DARKGRAY: c_uint =     0x08;
pub const CACA_LIGHTBLUE: c_uint =    0x09;
pub const CACA_LIGHTGREEN: c_uint =   0x0a;
pub const CACA_LIGHTCYAN: c_uint =    0x0b;
pub const CACA_LIGHTRED: c_uint =     0x0c;
pub const CACA_LIGHTMAGENTA: c_uint = 0x0d;
pub const CACA_YELLOW: c_uint =       0x0e;
pub const CACA_WHITE: c_uint =        0x0f;
pub const CACA_DEFAULT: c_uint =      0x10;
pub const CACA_TRANSPARENT: c_uint =  0x20;

// Styles
pub const CACA_BOLD: c_uint =      0x01;
pub const CACA_ITALICS: c_uint =   0x02;
pub const CACA_UNDERLINE: c_uint = 0x04;
pub const CACA_BLINK: c_uint =     0x08;

// Driver types
pub const CACA_DRIVER_NULL: c_uint    = 0;
pub const CACA_DRIVER_RAW: c_uint     = 1;
pub const CACA_DRIVER_COCOA: c_uint   = 2;
pub const CACA_DRIVER_CONIO: c_uint   = 3;
pub const CACA_DRIVER_GL: c_uint      = 4;
pub const CACA_DRIVER_NCURSES: c_uint = 5;
pub const CACA_DRIVER_SLANG: c_uint   = 6;
pub const CACA_DRIVER_VGA: c_uint     = 7;
pub const CACA_DRIVER_WIN32: c_uint   = 8;
pub const CACA_DRIVER_X11: c_uint     = 9;

// Event types
pub const CACA_EVENT_NONE: c_uint          = 0x0000;
pub const CACA_EVENT_KEY_PRESS: c_uint     = 0x0001;
pub const CACA_EVENT_KEY_RELEASE: c_uint   = 0x0002;
pub const CACA_EVENT_MOUSE_PRESS: c_uint   = 0x0004;
pub const CACA_EVENT_MOUSE_RELEASE: c_uint = 0x0008;
pub const CACA_EVENT_MOUSE_MOTION: c_uint  = 0x0010;
pub const CACA_EVENT_RESIZE: c_uint        = 0x0020;
pub const CACA_EVENT_QUIT: c_uint          = 0x0040;
pub const CACA_EVENT_ANY: c_uint           = 0xffff;

// Key values
pub const CACA_KEY_UNKNOWN: c_int = 0x00;
pub const CACA_KEY_CTRL_A: c_int =    0x01;
pub const CACA_KEY_CTRL_B: c_int =    0x02;
pub const CACA_KEY_CTRL_C: c_int =    0x03;
pub const CACA_KEY_CTRL_D: c_int =    0x04;
pub const CACA_KEY_CTRL_E: c_int =    0x05;
pub const CACA_KEY_CTRL_F: c_int =    0x06;
pub const CACA_KEY_CTRL_G: c_int =    0x07;
pub const CACA_KEY_BACKSPACE: c_int = 0x08;
pub const CACA_KEY_TAB: c_int =       0x09;
pub const CACA_KEY_CTRL_J: c_int =    0x0a;
pub const CACA_KEY_CTRL_K: c_int =    0x0b;
pub const CACA_KEY_CTRL_L: c_int =    0x0c;
pub const CACA_KEY_RETURN: c_int =    0x0d;
pub const CACA_KEY_CTRL_N: c_int =    0x0e;
pub const CACA_KEY_CTRL_O: c_int =    0x0f;
pub const CACA_KEY_CTRL_P: c_int =    0x10;
pub const CACA_KEY_CTRL_Q: c_int =    0x11;
pub const CACA_KEY_CTRL_R: c_int =    0x12;
pub const CACA_KEY_PAUSE: c_int =     0x13;
pub const CACA_KEY_CTRL_T: c_int =    0x14;
pub const CACA_KEY_CTRL_U: c_int =    0x15;
pub const CACA_KEY_CTRL_V: c_int =    0x16;
pub const CACA_KEY_CTRL_W: c_int =    0x17;
pub const CACA_KEY_CTRL_X: c_int =    0x18;
pub const CACA_KEY_CTRL_Y: c_int =    0x19;
pub const CACA_KEY_CTRL_Z: c_int =    0x1a;
pub const CACA_KEY_ESCAPE: c_int =    0x1b;
pub const CACA_KEY_DELETE: c_int =    0x7f;
pub const CACA_KEY_UP: c_int =    0x111;
pub const CACA_KEY_DOWN: c_int =  0x112;
pub const CACA_KEY_LEFT: c_int =  0x113;
pub const CACA_KEY_RIGHT: c_int = 0x114;
pub const CACA_KEY_INSERT: c_int =   0x115;
pub const CACA_KEY_HOME: c_int =     0x116;
pub const CACA_KEY_END: c_int =      0x117;
pub const CACA_KEY_PAGEUP: c_int =   0x118;
pub const CACA_KEY_PAGEDOWN: c_int = 0x119;
pub const CACA_KEY_F1: c_int =  0x11a;
pub const CACA_KEY_F2: c_int =  0x11b;
pub const CACA_KEY_F3: c_int =  0x11c;
pub const CACA_KEY_F4: c_int =  0x11d;
pub const CACA_KEY_F5: c_int =  0x11e;
pub const CACA_KEY_F6: c_int =  0x11f;
pub const CACA_KEY_F7: c_int =  0x120;
pub const CACA_KEY_F8: c_int =  0x121;
pub const CACA_KEY_F9: c_int =  0x122;
pub const CACA_KEY_F10: c_int = 0x123;
pub const CACA_KEY_F11: c_int = 0x124;
pub const CACA_KEY_F12: c_int = 0x125;
pub const CACA_KEY_F13: c_int = 0x126;
pub const CACA_KEY_F14: c_int = 0x127;
pub const CACA_KEY_F15: c_int = 0x128;

// Since the implementation details of these structs are hidden in the C
// interface of libcaca itself, these are just unit structs. All operations on
// them are done by passing them to the libcaca functions.
#[repr(C)]
pub struct CacaCanvasRaw;
#[repr(C)]
pub struct CacaDitherRaw;
#[repr(C)]
pub struct CacaCharfontRaw;
#[repr(C)]
pub struct CacaFontRaw;
#[repr(C)]
pub struct CacaDisplayRaw;

#[repr(C)]
pub struct CacaEventRaw {
    pub type_: c_uint,
    // the largest sized event in the union is the key event.
    // for now, just manually transmute event information based on event type.
    pub data: [u8; 2 + 2 + 4], // c_int + u32 + c_char[8]
}

extern "C" {
    // basic functions
    pub fn caca_create_canvas(w: c_int, h: c_int) -> *mut CacaCanvasRaw;
    pub fn caca_manage_canvas(cv: *mut CacaCanvasRaw, callback: Option<unsafe extern "C" fn (*mut c_void) -> *const c_int>, p: *mut c_void) -> c_int;
    pub fn caca_unmanage_canvas(cv: *mut CacaCanvasRaw, callback: Option<unsafe extern "C" fn (*mut c_void) -> *const c_int>, p: *mut c_void) -> c_int;
    pub fn caca_set_canvas_size(cv: *mut CacaCanvasRaw, w: c_int, h: c_int) -> c_int;
    pub fn caca_get_canvas_width(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_get_canvas_height(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_get_canvas_chars(cv: *const CacaCanvasRaw) -> *const u32;
    pub fn caca_get_canvas_attrs(cv: *const CacaCanvasRaw) -> *const u32;
    pub fn caca_free_canvas(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_get_version() -> *const c_char;

    // canvas drawing
    pub fn caca_gotoxy(cv: *mut CacaCanvasRaw, x: c_int, y: c_int) -> c_int;
    pub fn caca_wherex(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_wherey(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_put_char(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, c: u32) -> c_int;
    pub fn caca_get_char(cv: *const CacaCanvasRaw, x: c_int, y: c_int) -> u32;
    pub fn caca_put_str(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, str: *const c_char) -> c_int;
    // pub fn caca_prc_intf(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, str: *const c_char, ...) -> c_int;
    // pub fn caca_vprc_intf(cv: *mut CacaCanvasRaw, c_int, c_int, *const c_char, va_list) -> c_int;
    pub fn caca_clear_canvas(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_set_canvas_handle(cv: *mut CacaCanvasRaw, x: c_int, y: c_int) -> c_int;
    pub fn caca_get_canvas_handle_x(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_get_canvas_handle_y(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_blit(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, cv1: *const CacaCanvasRaw, cv2: *const CacaCanvasRaw) -> c_int;
    pub fn caca_set_canvas_boundaries(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int;

    // dirty rectangle manipulation
    pub fn caca_disable_dirty_rect(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_enable_dirty_rect(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_get_dirty_rect_count(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_get_dirty_rect(cv: *mut CacaCanvasRaw, idx: c_int, x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int) -> c_int;
    pub fn caca_add_dirty_rect(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int;
    pub fn caca_remove_dirty_rect(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int;
    pub fn caca_clear_dirty_rect_list(cv: *mut CacaCanvasRaw) -> c_int;

    // canvas transformation
    pub fn caca_invert(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_flip(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_flop(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_rotate_180(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_rotate_left(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_rotate_right(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_stretch_left(cv: *mut CacaCanvasRaw) -> c_int;
    pub fn caca_stretch_right(cv: *mut CacaCanvasRaw) -> c_int;

    // attribute conversions
    pub fn caca_get_attr(cv: *const CacaCanvasRaw, x: c_int, y: c_int) -> u32;
    pub fn caca_set_attr(cv: *mut CacaCanvasRaw, a: u32) -> c_int;
    pub fn caca_unset_attr(cv: *mut CacaCanvasRaw, a: u32) -> c_int;
    pub fn caca_toggle_attr(cv: *mut CacaCanvasRaw, a: u32) -> c_int;
    pub fn caca_put_attr(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, a: u32) -> c_int;
    pub fn caca_set_color_ansi(cv: *mut CacaCanvasRaw, fg: u8, bg: u8) -> c_int;
    pub fn caca_set_color_argb(cv: *mut CacaCanvasRaw, fg: u16, bg: u16) -> c_int;
    pub fn caca_attr_to_ansi(a: u32) -> u8;
    pub fn caca_attr_to_ansi_fg(a: u32) -> u8;
    pub fn caca_attr_to_ansi_bg(a: u32) -> u8;
    pub fn caca_attr_to_rgb12_fg(a: u32) -> u16;
    pub fn caca_attr_to_rgb12_bg(a: u32) -> u16;
    pub fn caca_attr_to_argb64(a: u32, argb: *mut [u8; 8]) -> ();

    // character set conversions
    // pub fn caca_utf8_to_utf32(*const c_char, size_t *) -> u32;
    // pub fn caca_utf32_to_utf8(char *, u32) -> size_t;
    // pub fn caca_utf32_to_cp437(u32) -> u8;
    // pub fn caca_cp437_to_utf32(u8) -> u32;
    // pub fn caca_utf32_to_ascii(u32) -> char;
    // pub fn caca_utf32_is_fullwidth(u32) -> int;

    // primitives drawing
    pub fn caca_draw_line(cv: *mut CacaCanvasRaw, x1: c_int, y1: c_int, x2: c_int, y2: c_int, c: u32) -> c_int;
    pub fn caca_draw_polyline(cv: *mut CacaCanvasRaw, x: *const c_int, y: *const c_int, n: c_int, c: u32) -> c_int;
    pub fn caca_draw_thin_line(cv: *mut CacaCanvasRaw, x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int;
    pub fn caca_draw_thin_polyline(cv: *mut CacaCanvasRaw, x: *const c_int, y: *const c_int, n: c_int) -> c_int;

    pub fn caca_draw_circle(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, r: c_int, c: u32) -> c_int;
    pub fn caca_draw_ellipse(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, a: c_int, b: c_int, c: u32) -> c_int;
    pub fn caca_draw_thin_ellipse(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, a: c_int, b: c_int) -> c_int;
    pub fn caca_fill_ellipse(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, a: c_int, b: c_int, c: u32) -> c_int;

    pub fn caca_draw_box(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int, c: u32) -> c_int;
    pub fn caca_draw_thin_box(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int;
    pub fn caca_draw_cp437_box(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int;
    pub fn caca_fill_box(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int, c: u32) -> c_int;

    pub fn caca_draw_triangle(cv: *mut CacaCanvasRaw, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int, c: u32) -> c_int;
    pub fn caca_draw_thin_triangle(cv: *mut CacaCanvasRaw, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int);
    pub fn caca_fill_triangle(cv: *mut CacaCanvasRaw, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int, c: u32) -> c_int;
    pub fn caca_fill_triangle_textured(cv: *mut CacaCanvasRaw, coords: [c_int; 6], tex: *mut CacaCanvasRaw, uv: [c_float; 6]) -> c_int;

    // canvas frame handling
    pub fn caca_get_frame_count(cv: *const CacaCanvasRaw) -> c_int;
    pub fn caca_set_frame(cv: *mut CacaCanvasRaw, f: c_int) -> c_int;
    pub fn caca_get_frame_name(cv: *const CacaCanvasRaw) -> *const char;
    pub fn caca_set_frame_name(cv: *mut CacaCanvasRaw, n: *const char) -> c_int;
    pub fn caca_create_frame(cv: *mut CacaCanvasRaw, f: c_int) -> c_int;
    pub fn caca_free_frame(cv: *mut CacaCanvasRaw, f: c_int) -> c_int;

    // // bitmap dithering
    pub fn caca_create_dither(bpp: c_int, w: c_int, h: c_int, pitch: c_int, rmask: u32, gmask: u32, bmask: u32, amask: u32) -> *mut CacaDitherRaw;
    pub fn caca_set_dither_palette(d: *mut CacaDitherRaw, r: *const u32, g: *const u32, b: *const u32, a: *const u32) -> c_int;
    pub fn caca_set_dither_brightness(d: *mut CacaDitherRaw, b: c_float) -> c_int;
    pub fn caca_get_dither_brightness(d: *const CacaDitherRaw) -> c_float;
    pub fn caca_set_dither_gamma(d: *mut CacaDitherRaw, g: c_float) -> c_int;
    pub fn caca_get_dither_gamma(d: *const CacaDitherRaw) -> c_float;
    pub fn caca_set_dither_contrast(d: *mut CacaDitherRaw, c: c_float) -> c_int;
    pub fn caca_get_dither_contrast(d: *const CacaDitherRaw) -> c_float;
    pub fn caca_set_dither_antialias(d: *mut CacaDitherRaw, s: *const c_char) -> c_int;
    pub fn caca_get_dither_antialias_list(d: *const CacaDitherRaw) -> *const *const c_char;
    pub fn caca_get_dither_antialias(d: *const CacaDitherRaw) -> *const c_char;
    pub fn caca_set_dither_color(d: *mut CacaDitherRaw, s: *const c_char) -> c_int;
    pub fn caca_get_dither_color_list(d: *const CacaDitherRaw) -> *const *const c_char;
    pub fn caca_get_dither_color(d: *const CacaDitherRaw) -> *const c_char;
    pub fn caca_set_dither_charset(d: *mut CacaDitherRaw, s: *const c_char) -> c_int;
    pub fn caca_get_dither_charset_list(d: *const CacaDitherRaw) -> *const *const c_char;
    pub fn caca_get_dither_charset(d: *const CacaDitherRaw) -> *const c_char;
    pub fn caca_set_dither_algorithm(d: *mut CacaDitherRaw, s: *const c_char) -> c_int;
    pub fn caca_get_dither_algorithm_list(d: *const CacaDitherRaw) -> *const *const c_char;
    pub fn caca_get_dither_algorithm(d: *const CacaDitherRaw) -> *const c_char;
    pub fn caca_dither_bitmap(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int, d: *const CacaDitherRaw, data: *const c_void) -> c_int;
    pub fn caca_free_dither(d: *mut CacaDitherRaw) -> c_int;

    // character font handling
    pub fn caca_load_charfont(data: *const c_void, len: size_t) -> *mut CacaCharfontRaw;
    pub fn caca_free_charfont(charfont: *mut CacaCharfontRaw) -> c_int;

    // bitmap font handling
    pub fn caca_load_font(data: *const c_void, len: size_t) -> *mut CacaFontRaw;
    pub fn caca_get_font_list() -> *const *const c_char;
    pub fn caca_get_font_width(font: *const CacaFontRaw) -> c_int;
    pub fn caca_get_font_height(font: *const CacaFontRaw) -> c_int;
    pub fn caca_get_font_blocks(font: *const CacaFontRaw) -> *const u32;
    pub fn caca_render_canvas(cv: *const CacaCanvasRaw, f: *const CacaFontRaw, buf: *mut c_void, w: c_int, h: c_int, pitch: c_int) -> c_int;
    pub fn caca_free_font(font: *mut CacaFontRaw) -> c_int;

    // FIGfont handling
    pub fn caca_canvas_set_figfont(cv: *mut CacaCanvasRaw, filename: *const c_char) -> c_int;
    pub fn caca_set_figfont_smush(cv: *mut CacaCanvasRaw, filename: *const c_char) -> c_int;
    pub fn caca_set_figfont_width(cv: *mut CacaCanvasRaw, w: c_int) -> c_int;
    pub fn caca_put_figchar(cv: *mut CacaCanvasRaw, c: u32) -> c_int;
    pub fn caca_flush_figlet(cv: *mut CacaCanvasRaw) -> c_int;

    // file IO
    // __extern caca_file_t *caca_file_open(*const c_char, const char *);
    // __extern int caca_file_close(caca_file_t *);
    // __extern uint64_t caca_file_tell(caca_file_t *);
    // __extern size_t caca_file_read(caca_file_t *, void *, size_t);
    // __extern size_t caca_file_write(caca_file_t *, const void *, size_t);
    // __extern char * caca_file_gets(caca_file_t *, char *, int);
    // __extern int caca_file_eof(caca_file_t *);

    // // importers/exporters from/to various formats
    pub fn caca_import_canvas_from_memory(cv: *mut CacaCanvasRaw, buf: *const c_void, len: size_t, fmt: *const c_char) -> ssize_t;
    pub fn caca_import_canvas_from_file(cv: *mut CacaCanvasRaw, filename: *const c_char, fmt: *const c_char) -> ssize_t;
    pub fn caca_import_area_from_memory(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, data: *const c_void, length: size_t, fmt: *const c_char) -> ssize_t;
    pub fn caca_import_area_from_file(cv: *mut CacaCanvasRaw, x: c_int, y: c_int, filename: *const c_char, fmt: *const c_char) -> ssize_t;
    pub fn caca_get_import_list() -> *const *const c_char;
    pub fn caca_export_canvas_to_memory(cv: *const CacaCanvasRaw, fmt: *const c_char, len: *mut size_t) -> *mut c_void;
    pub fn caca_export_area_to_memory(cv: *const CacaCanvasRaw, x: c_int, y: c_int, w: c_int, h: c_int, fmt: *const c_char, size: *mut size_t) -> *mut c_void;
    pub fn caca_get_export_list() -> *const *const c_char;

    // display functions
    pub fn caca_create_display(cv: *mut CacaCanvasRaw) -> *mut CacaDisplayRaw;
    pub fn caca_create_display_with_driver(cv: *mut CacaCanvasRaw, d: *const c_char) -> *mut CacaDisplayRaw;
    pub fn caca_get_display_driver_list() -> *const *const c_char;
    pub fn caca_get_display_driver(dv: *mut CacaDisplayRaw) -> *const c_char;
    pub fn caca_set_display_driver(dv: *mut CacaDisplayRaw, d: *const c_char) -> c_int;
    pub fn caca_free_display(dv: *mut CacaDisplayRaw) -> c_int;
    pub fn caca_get_canvas(dv: *mut CacaDisplayRaw) -> *mut CacaCanvasRaw;
    pub fn caca_refresh_display(dv: *mut CacaDisplayRaw) -> c_int;
    pub fn caca_set_display_time(dv: *mut CacaDisplayRaw, d: c_int) -> c_int;
    pub fn caca_get_display_time(dv: *const CacaDisplayRaw) -> c_int;
    pub fn caca_get_display_width(dv: *const CacaDisplayRaw) -> c_int;
    pub fn caca_get_display_height(dv: *const CacaDisplayRaw) -> c_int;
    pub fn caca_set_display_title(dv: *mut CacaDisplayRaw, title: *const c_char) -> c_int;
    pub fn caca_set_mouse(dv: *mut CacaDisplayRaw, flag: c_int) -> c_int;
    pub fn caca_set_cursor(dv: *mut CacaDisplayRaw, flag: c_int) -> c_int;

    // event handling
    pub fn caca_get_event(cv: *mut CacaDisplayRaw, event_mask: c_int, event: *mut CacaEventRaw, timeout: c_int) -> c_int;
    pub fn caca_get_mouse_x(cv: *const CacaDisplayRaw) -> c_int;
    pub fn caca_get_mouse_y(cv: *const CacaDisplayRaw) -> c_int;
    pub fn caca_get_event_type(cv: *const CacaEventRaw) -> c_uint;
    pub fn caca_get_event_key_ch(cv: *const CacaEventRaw) -> c_int;
    pub fn caca_get_event_key_utf32(cv: *const CacaEventRaw) -> u32;
    pub fn caca_get_event_key_utf8(cv: *const CacaEventRaw, utf8: *mut c_char) -> c_int;
    pub fn caca_get_event_mouse_button(cv: *const CacaEventRaw) -> c_int;
    pub fn caca_get_event_mouse_x(cv: *const CacaEventRaw) -> c_int;
    pub fn caca_get_event_mouse_y(cv: *const CacaEventRaw) -> c_int;
    pub fn caca_get_event_resize_width(cv: *const CacaEventRaw) -> c_int;
    pub fn caca_get_event_resize_height(cv: *const CacaEventRaw) -> c_int;

    // process management
    // pub fn caca_optind() -> int;
    // pub fn caca_optarg() -> *mut c_char;
    // pub fn caca_getopt(c_int, *mut *const c_char, *const c_char, struct caca_option const *, int *) -> int;
}
