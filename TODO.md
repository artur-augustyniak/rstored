#TODO
 - config file, na początek mogą być pliki do przekierowania z demonize lub duration sleep timeout param z pliku konfiguracyjnego i reload
- dynamiczne ładowanie pluginów `Operation`
- porządek z importami ::base

Podczas startu lub restartu demona
- tworzone są wbudowane pluginy df i mem oraz ładowane dodatkowe pluginy probe’y z katalogu zawartego w konfiguracji
- tworzone są i uruchamiane są wątki mechanizmów raportowania wg serwer i syslog
- Przy tworzeniu serwera są rejestrowane w api swagger jako endpointy handlery zarejestrowanych probe'ów. Handlery używają mapperów aby pobrać dane z bufora interesującego handler pluginu i przemapować na json
- Przy tworzeniu sysloga są rejestrowane handlery zarejestrowanych probów, Handlery przetwarzają dane z bufora danego probe'a na output syslog
- Syslog w pętli co timeout z konfiguracji odpytuje handlery zarejstrowanych probów

## Ogólne
- Repozytoria submoduły git dla pluginów probów
- Następnie aplikacja webowa używajaca klienta swagger może zarejestrować monitorowany serwer
- oddzielne repo na pluginy swagger client dla klienta (appka django i framework js)
- rstored kompilacja na inne platformy unikać systemd
- rstored - użycie komponentu w c i safe wrapper
- stored funkd - różne kompilacje
- stored memory monitor (napisany w rust)
- frontend do r/stored w kore
- frontend do r/stored w webowym frameworku rust http://mainisusuallyafunction.blogspot.com/2014/08/calling-rust-library-from-c-or-anything.html)
- https://bluishcoder.co.nz/2013/08/08/linking_and_calling_rust_functions_from_c.html