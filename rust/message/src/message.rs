use ::libc::c_char;
use ::string::FromChar;
use ::string::ByteSlice;

pub struct Message {
    text: String,
}

#[no_mangle]
pub extern fn message_new(string: *const c_char) -> *mut Message {
    let message = Message{
        text: String::from_char(string).clone(),
    };
    
    let boxed_data = Box::new(message);
    Box::into_raw(boxed_data)
}

#[no_mangle]
pub unsafe extern fn message_destroy(data: *mut Message) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern fn message_get_text(message: *const Message) -> ByteSlice {
    let message = &*message;
    ByteSlice::from(message.text.as_ref())
}
