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
        state_name = feature['properties']['NAME_1'].replace('/', '-')
        output_filename = f"{output_folder}/{state_name}.json"

        new_feature_collection = {
            "type": "FeatureCollection",
            "features": [feature]
        }

        with open(output_filename, 'w') as out_f:
            geojson.dump(new_feature_collection, out_f)

if __name__ == "__main__":
    # Get all files that end with _1.json
    files = glob.glob('*_1.json')
    for file in files:
        split_geojson(file)