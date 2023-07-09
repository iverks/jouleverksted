# Jouleverksted 🎅


For å laste env vars. Må gjøres hver gang ny terminal åpnes.

```powershell
~\export-esp.ps1
```

## Ressurser

Nesten alle har examples-mapper

Smart-led driver: <https://github.com/cat-in-136/ws2812-esp32-rmt-driver>

Hardware abstraction layer: <https://github.com/esp-rs/esp-idf-hal>

Installasjonsguide: <https://esp-rs.github.io/book/>

Eksempelprosjekt som dessverre er forvirrende af: <https://github.com/ivmarkov/rust-esp32-std-demo>

## Tutorial (windows) 

Merk: Du må ha mikrokontrolleren **esp32**

Disse instruksjonene er per nå utestet, men jeg tror det var slik jeg gjorde det.

1. Last ned og installer driver for esp32
   1. Last ned driver fra <https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers?tab=downloads>. Vet ikke om alle funker, men "CP210x Universal Windows Driver" fungerte for meg.
   2. Pakk ut .zip-filen
   3. Start enhetsbehandling (device manager)
   4. Koble arduino til PC og Legg merke til ny tilkobling i enhetsbehandling (Heter trolig noe som USB-to-UART bridge) 
   5. Høyreklikk på sistnevnte og trykk på oppdater driver.
   6. Velg så "bla gjennom min datamaskin" for drivere. Finn mappen du pakket ut tidligere og velg denne (du skal altså velge hele mappen, ikke en fil).
   7. Det skal nå ha blitt opprettet en ny tilkobling under Porter som for meg heter Silicon Labs CP210x USB to UART Bridge (COM4).
2. Last ned rust
   1. Last ned Visual Studio Build Tools om du ikke har de. <https://aka.ms/vs/17/release/vs_BuildTools.exe>
      1. Velg "Desktop Development with C++", (og under "Individual Components" huk av for "C++ Clang compiler" (usikker på om den siste er nødvendig))
   2. Last ned rust (`rustup-init.exe` for 64 bit windows) fra [hjemmesiden](https://www.rust-lang.org/learn/get-started) eller bruk [direkte nedlastningslenke](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe).
   3. Kjør sistnevnte program. Velg default instillinger.
   4. Se at det fungerer i terminalen `cargo --version`.
3. Konfigurer Python etter krav:
   1. [Merk: Kan ha problemer med python > 3.10](https://github.com/cs01/gdbgui/issues/447)
   2. Installer Python. Jeg anbefaler å installere fra nettsiden.
   3. Kjør `py -c "import os, sys; print(os.path.dirname(sys.executable))"`.
   4. Legg output fra forrige steg til PATH-variabelen din.
4. Last ned _nightly_ toolchain for rust: `rustup toolchain add nightly`
5. Last ned ldproxy: `cargo install ldproxy`
6. Last ned espup:
   1. `cargo install espup`
   2. `espup install`

Installasjon ferdig.

## Prosedyre for å kompilere og flashe

1. Åpne powershell. Du er da per default i `~`.
2. Kjør `~\export-esp.ps1` (eller `.\export-esp.ps1`). Det holder ofte å skrive `~\exp` og så trykke `TAB`
3. cd til denne mappen, f.eks. `cd ~\jouleverksted`
4. Kjør vscode `code .`
5. Optional: Kjør `cargo build` for å spare deg for kompileringstid på dependencies senere. Kommer til å throwe errors med det går bra.
6. Kopier `.default.env` til `.env`, og skriv inn ditt wi-fi navn og passord.
7. Gjør andre endringer om ønsket.
8. Kompiler og flash automatisk med `cargo run`. Første gang vil det ta lang tid.

## Bruker-"dokumentasjon"

GET-"10.0.0.22/" for å sjekke at den lever. NB: vet ikke hvor stabil ipn er.
GET-"/off" Skru av
GET-"/rotate" Bytt til neste program i lista.