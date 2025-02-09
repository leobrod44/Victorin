import undetected_chromedriver as uc
from selenium import webdriver
import time
import urllib
import os
from selenium.webdriver.common.by import By

def create_directory(folder_path):
    if not os.path.exists(folder_path):
        os.makedirs(folder_path)

def download_images(plant_name, num_images):
    # Set up the driver
    driver_path = "/opt/homebrew/bin/chromedriver"
    options = webdriver.ChromeOptions()
    options.add_argument('--user-data-dir=/Users/leobrodeur/Library/Application Support/Google/Chrome/Default')
    options = webdriver.ChromeOptions()
    options.add_argument('--disable-blink-features=AutomationControlled')  # Disable automation detection
    options.add_argument('--start-maximized')  # Open browser in maximized mode
    options.add_argument('--disable-extensions')  # Disable any extensions
    options.add_argument('--no-sandbox')  # Disable sandbox mode
    options.add_argument('--disable-dev-shm-usage')  # Disable shared memory usage
    options.add_argument('--disable-gpu')  # Disable GPU hardware acceleration
    # You can also add a random user agent to mimic a real browser
    options.add_argument('user-agent=Your custom user agent string')
    driver = uc.Chrome(options=options)

    # Create the directory for the plant
    folder_path = f'/Users/leobrodeur/Victorin/ml/dataset/{plant_name.replace(" ", "_")}/'
    create_directory(folder_path)

    # Build the search URL dynamically based on the plant name
    search_url = f'https://www.google.com/search?q={plant_name.replace(" ", "+")}&tbm=isch'
    # search_url =  'https://www.google.com/search?sca_esv=360ab29c550ae11e&sxsrf=AHTn8zq1NhQh_UVJEfaa__Tp9Dou8jfymg:1739071025712&q=monstera+deliciosa+in+pot&udm=2&fbs=ABzOT_CWdhQLP1FcmU5B0fn3xuWpA-dk4wpBWOGsoR7DG5zJBtmuEdhfywyzhendkLDnhcq4Fx59GJo42IHin57PCNq1BCLfVKQIWPymDGFj2PR-DaOFXu-BnsGeOvX43fYDTIQk3NefR4EEb-ebM3m4QCHpn8F3hj8loZf8UVS7gA7pesLbGCPGRAGgIdD_aRz5J-MW5r2OoZ_j2z7Hvvawu8qipWEekw&sa=X&ved=2ahUKEwjp8-e10LWLAxWXFlkFHeKZBRwQtKgLegQIFBAB'
    driver.get(search_url)
    time.sleep(1)

    # Locate all image elements with the exact class "YQ4gaf"
    img_result = driver.find_elements(By.XPATH, "//img[contains(@class, 'YQ4gaf') and not(contains(@class, ' '))]")  # Match exact class "YQ4gaf"
    img_urls = []

    # Click on each image to open it and extract the full-resolution URL
    for index, img in enumerate(img_result[:num_images]):  # Adjust the range based on user input
        try:
            # Click the image to open it
            img.click()
            time.sleep(3)  # Allow time for the modal to load
            # Extract the high-resolution image URL from the opened modal
            large_img = driver.find_element(By.XPATH, "//img[@jsname='kn3ccd']")
            img_url = large_img.get_attribute('src')

            # Append the URL to the list
            img_urls.append(img_url)

            # Save the image
            urllib.request.urlretrieve(img_url, folder_path + f"image_{index + 1}.jpg")

            # Close the modal (Click outside or press escape, depending on the site)
            driver.execute_script("document.querySelector('[aria-label=\"Close\"]').click()")
            time.sleep(1)

        except Exception as e:
            print(f"Error with image {index + 1}")
            continue

    driver.quit()

def main():
    # Define the list of plant names
    # done 'Jade plant (Crassula ovata)', 'Rubber Plant (Ficus elastica)', 'Schefflera', 'Areca Palm (Dypsis lutescens)'
    plants = ['Asparagus Fern (Asparagus setaceus)', 'Iron Cross begonia (Begonia masoniana)', 'Lily of the valley (Convallaria majalis)', 'Prayer Plant (Maranta leuconeura)', 'Dracaena', 'Aloe Vera', 'Begonia (Begonia spp.)', 'Kalanchoe', 'Lilium (Hemerocallis)', 'Pothos (Ivy arum)', 'Polka Dot Plant (Hypoestes phyllostachya)', 'Yucca', 'Dumb Cane (Dieffenbachia spp.)', 'Daffodils (Narcissus spp.)', 'Elephant Ear (Alocasia spp.)', 'Poinsettia (Euphorbia pulcherrima)', 'Calathea', 'Monstera Deliciosa (Monstera deliciosa)', 'Hyacinth (Hyacinthus orientalis)', 'Sago Palm (Cycas revoluta)', 'Chrysanthemum', 'Ponytail Palm (Beaucarnea recurvata)', 'Anthurium (Anthurium andraeanum)', 'Tradescantia', 'Chinese Money Plant (Pilea peperomioides)', 'Chinese evergreen (Aglaonema)', 'Tulip', 'Parlor Palm (Chamaedorea elegans)', 'Peace lily', 'ZZ Plant (Zamioculcas zamiifolia)', 'Venus Flytrap', 'Christmas Cactus (Schlumbergera bridgesii)', 'Rattlesnake Plant (Calathea lancifolia)', 'Money Tree (Pachira aquatica)', 'Boston Fern (Nephrolepis exaltata)', 'Cast Iron Plant (Aspidistra elatior)', 'Orchid', 'African Violet (Saintpaulia ionantha)', 'Ctenanthe', 'Snake plant (Sanseviera)', 'Bird of Paradise (Strelitzia reginae)', 'English Ivy (Hedera helix)', 'Birds Nest Fern (Asplenium nidus)']

    plant_names = [name+" in pot" for name in plants]
    num_images = 60
    # Prompt the user for the number of images per plant

    # Loop through each plant name and download images
    for plant in plant_names[:20]:
        print(f"Starting image download for {plant}...")
        download_images(plant, num_images)
        print(f"Images for {plant} have been downloaded.")

if __name__ == "__main__":
    main()
