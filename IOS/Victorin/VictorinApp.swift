//
//  VictorinApp.swift
//  Victorin
//
//  Created by Leo Brodeur on 2025-01-23.
//

import SwiftUI
@main
struct VictorinApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    let persistenceController = PersistenceController.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(\.managedObjectContext, persistenceController.container.viewContext)
        }
    }
}
