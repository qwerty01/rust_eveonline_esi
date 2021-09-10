#!/bin/bash

openapi-generator generate \
  -i 'https://esi.evetech.net/latest/swagger.json?datasource=tranquility' \
  -g rust \
  --library reqwest \
  --additional-properties=supportMultipleResponses=true,useSingleRequestParameter=true,packageName=rust_eveonline_esi
