import os
import glob
import geojson

def split_geojson(input_file):
    # Extract the country code from the input file name
    country_code = os.path.basename(input_file).split('_')[0]
    output_folder = country_code
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)

    with open(input_file) as f:
        data = geojson.load(f)

    for feature in data['features']:
        # Use 'shapeName' instead of 'NAME_1' to get the state name
        state_name = feature['properties']['shapeName'].replace('/', '-')
        output_filename = f"{output_folder}/{state_name}.json"

        new_feature_collection = {
            "type": "FeatureCollection",
            "features": [feature]
        }

        with open(output_filename, 'w') as out_f:
            geojson.dump(new_feature_collection, out_f)

if __name__ == "__main__":
    # Get all files that end with _1.json
    files = glob.glob('*ADM1_simplified.json') or glob.glob('*ADM1_simplified.geojson')
    for file in files:
        split_geojson(file)