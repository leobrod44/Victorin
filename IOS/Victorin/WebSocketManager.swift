//
//  WebSocketManager.swift
//  Victorin
//
//  Created by Leo Brodeur on 2025-01-27.
//

import Foundation

class WebSocketManager: ObservableObject {
    static let shared = WebSocketManager()
    private var webSocketTask: URLSessionWebSocketTask?
    private let url = URL(string: "ws://192.168.0.15:3031/humidity_updates")! // Update with your server's WebSocket URL
    
    @Published var humidityUpdates: [Int: Double] = [:] // Dictionary to store humidity by device ID

    private init() {
        connect()
    }

    func connect() {
        let session = URLSession(configuration: .default)
        webSocketTask = session.webSocketTask(with: url)
        webSocketTask?.resume()
        receiveMessages()
    }

    func disconnect() {
        webSocketTask?.cancel(with: .goingAway, reason: nil)
    }

    private func receiveMessages() {
        webSocketTask?.receive { [weak self] result in
            switch result {
            case .failure(let error):
                print("WebSocket error: \(error)")
                self?.connect() // Reconnect on failure
            case .success(let message):
                switch message {
                case .data(let data):
                    self?.handleMessage(data: data)
                case .string(let text):
                    if let data = text.data(using: .utf8) {
                        self?.handleMessage(data: data)
                    }
                @unknown default:
                    break
                }
            }
            self?.receiveMessages()
        }
    }

    private func handleMessage(data: Data) {
        do {
            let update = try JSONDecoder().decode(HumidityUpdate.self, from: data)
            DispatchQueue.main.async {
                self.humidityUpdates[update.id] = update.humidity
            }
        } catch {
            print("Failed to decode humidity update: \(error)")
        }
    }
}

struct HumidityUpdate: Decodable {
    let id: Int
    let humidity: Double
}
