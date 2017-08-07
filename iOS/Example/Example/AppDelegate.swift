//
//  AppDelegate.swift
//  Example
//
//  Created by Wojtek on 07/08/2017.
//  Copyright © 2017 Wojtek. All rights reserved.
//

import UIKit

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?) -> Bool {

        let message = Message(text: "some text")
        print(message.name)
        
        return true
    }
}
