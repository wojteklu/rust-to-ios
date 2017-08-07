//
//  Message.swift
//  Example
//
//  Created by Wojtek on 07/08/2017.
//  Copyright © 2017 Wojtek. All rights reserved.
//

import Foundation

class Message {
    private let raw: OpaquePointer
    
    init(text: String) {
        raw = message_new(text)
    }
    
    deinit {
        message_destroy(raw)
    }
    
    var name: String {
        let byteSlice = message_get_text(raw)
        return byteSlice.toString()!
    }
}

extension ByteSlice {
    
    func toString(encoding: String.Encoding = String.Encoding.utf8) -> String? {
        let pointer = UnsafeBufferPointer<UInt8>(start: bytes, count: length)
        return String(bytes: pointer, encoding: encoding)
    }
}
