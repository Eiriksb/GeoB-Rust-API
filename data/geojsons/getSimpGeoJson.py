from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.alert import Alert
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import os

# Set up the WebDriver for Safari
options = webdriver.SafariOptions()
options.set_capability("safari.options.setAutomaticPromptAlert", True)
driver = webdriver.Safari(options=options)

# Navigate to the page
driver.get("https://www.geoboundaries.org/simplifiedDownloads.html")

# Wait for the table to load
# (Assuming you have a way to wait for the table to load, e.g., by waiting for a specific element)

# Find the download link and click it
download_link = driver.find_element(By.CSS_SELECTOR, 'div.tabulator-cell[tabulator-field="staticDownloadLink"] i.fa.fa-download')
download_link.click()

# Wait for the download permission dialog to appear
# (You might need to implement a custom wait or check for the presence of the dialog)

# Accept the download permission dialog
alert = Alert(driver)
alert.accept()

# Close the browser
driver.quit()