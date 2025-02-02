import CoreData
import SwiftUI

struct ContentView: View {
    @State private var plants: [Plant] = [
        Plant(device: 1, name: "Pothos", cycle: 7, duration: 10),
        Plant(device: 2, name: "Fern", cycle: 5, duration: 5)
    ]
    
    @State private var isEditing = false
    @State private var isAdding = false // To track if we are adding a new plant
    @State private var selectedPlantIndex: Int? = nil
    @State private var newPlantName: String = "" // State for new plant form fields
    @State private var newPlantCycle: Int = 1
    @State private var newPlantDuration: Int = 5
    
    let columns = [
        GridItem(.flexible(), spacing: 10),
        GridItem(.flexible(), spacing: 10)
    ]
    
    var body: some View {
        NavigationView {
            ZStack {
                ScrollView {
                    LazyVGrid(columns: columns, spacing: 20) {
                        ForEach(plants.indices, id: \.self) { index in
                            PlantView(plant: $plants[index], onEdit: {
                                self.selectedPlantIndex = index
                                self.isEditing = true
                            })
                            .frame(minHeight: 250)
                        }
                    }
                    .padding()
                }
                
                // Add button at bottom right
                VStack {
                    Spacer()
                    HStack {
                        Spacer()
                        Button(action: {
                            self.isAdding = true
                        }) {
                            Circle()
                                .fill(Color.blue)
                                .frame(width: 60, height: 60)
                                .overlay(
                                    Text("+")
                                        .font(.largeTitle)
                                        .foregroundColor(.white)
                                )
                                .shadow(radius: 5)
                        }
                        .padding()
                    }
                }
            }
            .sheet(isPresented: $isAdding) {
                AddPlantForm(isAdding: $isAdding, plants: $plants)
            }
            .sheet(isPresented: $isEditing) {
                if let index = selectedPlantIndex {
                    //                    EditPlantForm(plant: $plants[index])
                }
            }
        }
    }
    
    private func clearAddPlantFields() {
        newPlantName = ""
        newPlantCycle = 1
        newPlantDuration = 5
    }
    
}


struct PlantView: View {
    @Binding var plant: Plant
    var onEdit: () -> Void // Closure to handle edit'
    
    @ObservedObject private var webSocketManager = WebSocketManager.shared
    
    @State private var isWatering = false // Tracks watering state for the animation
    @State private var humidity: Double = 0.0 // Initial humidity value
    
    var body: some View {
        GeometryReader { geometry in
            VStack {
                HStack {
                    Text(plant.name)
                        .font(.headline)
                        .padding(.top, 5)
                        .frame(maxWidth: .infinity, alignment: .center)
                    
                    Spacer()
                    
                    Button(action: onEdit) {
                        Image(systemName: "ellipsis")
                            .foregroundColor(.white)
                            .font(.title2)
                            .padding(10)
                            .background(Color.gray.opacity(0.4))
                            .clipShape(Circle())
                    }
                    .padding(.top, 4)
                    .padding(.trailing, 2)
                }
                
                ZStack {
                    HStack {
                        Spacer()
                        Image("plant")
                            .resizable()
                            .scaledToFit()
                            .frame(width: geometry.size.width * 0.55, height: geometry.size.height * 0.55)
                            .background(Color.white)
                            .cornerRadius(10)
                        Spacer()
                    }
                }
                
                Spacer()
                
                HStack {
                    ZStack {
                        Circle()
                            .fill(Color.cyan)
                            .frame(width: 50, height: 50) // Increased size for better readability
                        
                        Text("\(humidity)%") // Display humidity inside the circle
                            .font(.caption)
                            .foregroundColor(.white)
                    }
                    
                    Button(action: {
                        triggerWatering(plant: plant)
                    }) {
                        if isWatering {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle())
                                .frame(maxWidth: .infinity)
                        } else {
                            Text("Water")
                                .frame(maxWidth: .infinity)
                        }
                    }
                    .padding(.vertical, 5)
                    .background(isWatering ? Color.gray : Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
                }
                .padding(.top, 10)
            }
            .padding()
            .overlay(
                RoundedRectangle(cornerRadius: 15)
                    .stroke(Color.gray, lineWidth: 1)
            ).onAppear {
                // Update the humidity from the WebSocket when the view appears
                updateHumidity()
            }
            .onChange(of: webSocketManager.humidityUpdates) { _ in
                updateHumidity()
            }
        }
        .frame(minHeight: 250)
    }
    
    func updateHumidity() {
        if let updatedHumidity = webSocketManager.humidityUpdates[plant.device] {
            humidity = updatedHumidity
        }
    }
    
    func triggerWatering(plant: Plant) {
        
        guard let url = URL(string: "http://127.0.0.1:3030/water_plant") else {
            print("Invalid URL")
            isWatering = false
            return
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        let body: [String: Any] = [
            "id": plant.device,
            "name": plant.name
        ]
        
        do {
            request.httpBody = try JSONSerialization.data(withJSONObject: body, options: [])
        } catch {
            print("Failed to encode JSON")
            isWatering = false
            return
        }
        
        URLSession.shared.dataTask(with: request) { data, response, error in
            DispatchQueue.main.async {
                if let error = error {
                    print("Error: \(error.localizedDescription)")
                } else if let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 {
                    isWatering = true // Start loading animation
                    print("Watering triggered successfully!")
                    
                    // Simulate waiting for the Raspberry Pi response
                    DispatchQueue.main.asyncAfter(deadline: .now() + Double(plant.duration)) {
                        isWatering = false // End loading animation
                    }
                } else {
                    print("Failed to water plant.")
                    isWatering = false
                }
            }
        }.resume()
    }
}



struct AddPlantForm: View {
    @Binding var isAdding: Bool
    @Binding var plants: [Plant]
    
    @State private var newPlantName: String = "Plant"
    @State private var newDevice: String = ""
    @State private var parsedDevice: Int? = nil
    @State private var newPlantCycle: Int = 7
    @State private var newPlantDuration: Int = 10
    
    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("New Plant Details")) {
                    TextField("Name:", text: $newPlantName)
                    
                    TextField("Device:", text: Binding(
                        get: {
                            newDevice
                        },
                        set: { newValue in
                            newDevice = newValue
                            if let intValue = Int(newValue) {
                                parsedDevice = intValue
                            } else {
                                parsedDevice = nil
                            }
                        }
                    ))
                    
                    // Stepper for Watering Cycle (in days)
                    Stepper("Watering Cycle: \(newPlantCycle) days", value: $newPlantCycle, in: 1...60)
                        .padding(.vertical, 5)
                    
                    // Stepper for Duration (in seconds)
                    Stepper("Duration: \(newPlantDuration) seconds", value: $newPlantDuration, in: 1...3600)
                        .padding(.vertical, 5)
                }
            }
            .navigationBarItems(leading: Button("Cancel") {
                isAdding = false // Close the add form
            }, trailing: Button("Save") {
                // Add the plant when saving
                guard let device = parsedDevice,
                      let parsedDevice = parsedDevice else {
                    print("Failed to unwrap optional values")
                    return
                }
                let newPlant = Plant(device: device, name: newPlantName, cycle: newPlantCycle, duration: newPlantDuration)
                plants.append(newPlant)
                isAdding = false
            })
            .navigationBarTitle("Add New Plant", displayMode: .inline)
        }
    }
}



struct Plant: Identifiable {
    var id = UUID()
    var device: Int
    var name: String
    var cycle: Int // In days
    var duration: Int // In seconds
    var humidity: Double = 0.0 // New property for humidity
    var isWatering: Bool = false // New property for loading state
}

