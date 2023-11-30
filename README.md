# inx-edcas-indexer
Indexer for the edcas network

## Environment Variables

| Variable          | Example                                                               | Description                                                                                            |
|-------------------|-----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------|
| NUM_OF_WORKERS    | 4                                                                     | How many workers there are for pow. Usually this module doesn't need any computing power. Default is 4 |
| NODE_URL          | https://api.edcas.de                                                  | Url to your node                                                                                       |
| DATABASE_PORT     | 5432                                                                  |                                                                                                        |
| DATABASE_HOST     | localhost                                                             |                                                                                                        |
| DATABASE_NAME     | edcas                                                                 |                                                                                                        |
| POSTGRES_USER     | edcas                                                                 |                                                                                                        |
| POSTGRES_PASSWORD | dbpassword                                                            |                                                                                                        |
| EDDN_PUBLIC_KEY   | 0x00000...                                                            | Public key from inx-eddn module you trust                                                              |
| TAGS              | EDDN,SCAN,FSDJUMP,LOCATION,CARRIERJUMP,FSSBODYSIGNALS,SAASIGNALSFOUND | Tags the indexer should index                                                                          |
