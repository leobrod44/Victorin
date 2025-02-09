import undetected_chromedriver as uc
from selenium import webdriver
import time
import os
import csv
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

def create_directory(folder_path):
    try:
        if not os.path.exists(folder_path):
            os.makedirs(folder_path)
            print(f"Directory {folder_path} created.")
    except Exception as e:
        print(f"Error creating directory {folder_path}: {e}")

def download_plant_info(plant_name, writer, file):
    try:
        # Set up the driver
        driver_path = "/opt/homebrew/bin/chromedriver"
        options = webdriver.ChromeOptions()
        options.add_argument('--user-data-dir=/Users/leobrodeur/Library/Application Support/Google/Chrome/Default')
        options.add_argument('--disable-blink-features=AutomationControlled')  # Disable automation detection
        options.add_argument('--start-maximized')  # Open browser in maximized mode
        options.add_argument('--disable-extensions')  # Disable any extensions
        options.add_argument('--no-sandbox')  # Disable sandbox mode
        options.add_argument('--disable-dev-shm-usage')  # Disable shared memory usage
        options.add_argument('--disable-gpu')  # Disable GPU hardware acceleration
        options.add_argument('user-agent=Your custom user agent string')  # Optional custom user-agent
        driver = uc.Chrome(options=options)

        # Build the search URL dynamically based on the plant name
        search_url = f'https://www.missouribotanicalgarden.org/PlantFinder/PlantFinderProfileResults.aspx?basic={plant_name.replace(" ", "%20")}'
        driver.get(search_url)
        # Locate the search input and submit the plant name

        first_result = driver.find_element(By.XPATH, "//table[@class='results']//tr[1]//a")
        first_result.click()
        time.sleep(2)

        # Extract information from the plant's detailed page
        try:
            # Extracting plant information
            plant_info = {}
            rows = driver.find_elements(By.XPATH, "//div[@class='column-right']//div[@class='row']")
            for row in rows:
                columns = row.text.split(":")
                if len(columns) == 2:
                    key = columns[0].strip()
                    value = columns[1].strip()
                    plant_info[key] = value

            # Write the plant info to CSV
            row_data = [plant_name]  # Add the plant title first
            for key in ["Common Name", "Type", "Family", "Native Range", "Zone", "Height", "Spread", "Bloom Time", "Bloom Description", 
                        "Sun", "Water", "Maintenance", "Flower", "Leaf", "Tolerate"]:
                row_data.append(plant_info.get(key, "N/A"))
            print(row_data)
            writer.writerow(row_data)
            file.flush()  # Explicitly flush the buffer to ensure the data is written to the file
            
            # Quit the driver
            driver.quit()
        
        except Exception as e:
            print(f"Error extracting plant information for {plant_name}: {e}")
        
        # Quit the driver
        driver.quit()
    
    except Exception as e:
        print(f"Error during the process for {plant_name}: {e}")

def main():
    # Define the dataset folder path
    dataset_folder = './ml/dataset'
    create_directory(dataset_folder)  # Ensure the dataset folder exists

    # Create the CSV file and writer
    try:
        csv_file_path = os.path.join(dataset_folder, 'plant_data.csv')
        with open(csv_file_path, mode='w', newline='', encoding='utf-8') as file:
            writer = csv.writer(file)
            # Write the header row
            header = ["Common Name", "Type", "Family", "Native Range", "Zone", "Height", "Spread", "Bloom Time", "Bloom Description", 
                        "Sun", "Water", "Maintenance", "Flower", "Leaf", "Tolerate"]
            writer.writerow(header)
            file.flush()  # Ensure the header is immediately written

            # List of plants to scrape
            plants = ['Crassula ovata', 'Ficus elastica', 'Monstera Deliciosa', 'Aloe Vera']

            for plant in plants:
                try:
                    print(f"Scraping data for {plant}...")
                    download_plant_info(plant, writer, file)  # Pass file object to the function
                    print(f"Data for {plant} has been scraped.")
                except Exception as e:
                    print(f"Error scraping data for {plant}: {e}")
        
    except Exception as e:
        print(f"Error creating CSV file: {e}")

if __name__ == "__main__":
    main()
