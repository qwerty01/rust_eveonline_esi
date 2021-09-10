#!/bin/bash

openapi-generator generate \
  -i https://esi.evetech.net/latest/swagger.json?datasource=tranquility \
  -g rust \
  "--additional-properties=pubAuthor=LokiVKlokeNaAndoke,pubHomepage=https://github.com/LokiVKlokeNaAndoke/rust_eveonline_esi,pubName=rust_eveonline_esi,pubVersion=1.0.1"
