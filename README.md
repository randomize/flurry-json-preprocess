# DEPRECATED! flurry-json-preprocess

Rust utility to preprocess RAW data exported from Flurry into format more sutable for Pandas import
*DEPRECATED* since you can use lines mode in Pandas `data_df = pd.read_json('my.json', lines=True)` and avoid pre-processing

# How it works

Unfortunately Flurry RAW data exports are just bunch of separate JSON object (representing events), one per line, which makes it not really a syntacticly correct JSON file.  
This super simple utility just adds a `,` at the end of each line as well as the prefix `{ "data": [` and suffix `]}`, but does it as fast as I could get (using buffering and operating with bytes, not strings).

# Usage

- Export your Flurry events data to JSON format
- Feed the data into `flurry-jsonify`
- Import in Pandas as usual
- Enjoy ;)

# TODO

- Better error handling and Rust style
