import CoreData
import SwiftUI

struct ContentView: View {
    @State private var plants: [Plant] = []
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

    private func addPlant() {
        // Adding the new plant to the list
        let newPlant = Plant(name: newPlantName, cycle: newPlantCycle, duration: newPlantDuration)
        plants.append(newPlant)
        clearAddPlantFields() // Reset form fields after adding
    }

    private func clearAddPlantFields() {
        newPlantName = ""
        newPlantCycle = 1
        newPlantDuration = 5
    }
}


struct PlantView: View {
    @Binding var plant: Plant
    var onEdit: () -> Void // Closure to handle edit

    var body: some View {
        GeometryReader { geometry in
            VStack {
                HStack {
                    // Plant Name at the Top
                    Text(plant.name)
                        .font(.headline)
                        .padding(.top, 5)
                        .frame(maxWidth: .infinity, alignment: .center)

                    Spacer()

                    // Edit Button (3 Dots) at the Right of the Name
                    Button(action: onEdit) {
                        Image(systemName: "ellipsis")
                            .foregroundColor(.white)
                            .font(.title2)
                            .padding(10)
                            .background(Color.gray.opacity(0.4)) // Lighter alpha
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
                            .frame(width: geometry.size.width * 0.55, height: geometry.size.height * 0.55) // Use screen percentage
                            .background(Color.white)
                            .cornerRadius(10)
                        Spacer()
                    }
                }

                Spacer()

                HStack {
                    // Humidity Circle
                    Circle()
                        .fill(Color.cyan)
                        .frame(width: 20, height: 20)

                    // Water Button
                    Button("Water") {
                        // Trigger watering
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 5)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
                }
                .padding(.top, 10)
            }
            .padding()
            .overlay(
                RoundedRectangle(cornerRadius: 15)
                    .stroke(Color.gray, lineWidth: 1)
            )
        }
        .frame(minHeight: 250)
    }
}

struct AddPlantForm: View {
    @Binding var isAdding: Bool
    @Binding var plants: [Plant] // Reference to the plants array

    @State private var newPlantName: String = "Plant"
    @State private var newPlantCycle: Int = 7 // Default to 7 days
    @State private var newPlantDuration: Int = 3600 // Default to 3600 seconds (1 hour)

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("New Plant Details")) {
                    TextField("Name", text: $newPlantName)

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
                let newPlant = Plant(name: newPlantName, cycle: newPlantCycle, duration: newPlantDuration)
                plants.append(newPlant)
                isAdding = false // Close the form
            })
            .navigationBarTitle("Add New Plant", displayMode: .inline)
        }
    }
}


//struct EditPlantForm: View {
//    @Binding var plant: Plant
//
//    var body: some View {
//        NavigationView {
//            Form {
//                Section(header: Text("Edit Plant")) {
//                    TextField("Name", text: $plant.name)
//                    Stepper("Watering Cycle: \(plant.cycle)", value: $plant.cycle, in: 1...7)
//                    Stepper("Duration: \(plant.duration) hours", value: $plant.duration, in: 1...24)
//                }
//            }
//            .navigationBarItems(trailing: Button("Save") {
//                // No need to manually assign values here, as we are binding directly to the plant
//            })
//            .navigationBarTitle("Edit Plant", displayMode: .inline)
//        }
//    }
//}

struct Plant: Identifiable {
    var id = UUID()
    var name: String
    var cycle: Int // In days
    var duration: Int // In seconds
}
