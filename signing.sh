#! /bin/bash

# Al ejecutar el script debe estar en el mismo directorio que el archivo

# ------------------------------
# Proceso del firmado con PGP
asc_filename="$1.asc"

echo "Firmando Documento ..."
sleep 1
gpg --detach-sign --armor -o "$asc_filename" "$1"


echo -e "\n \nVerificando Firma del Documento... \n"
sleep 2
gpg --verify "$asc_filename" "$1" # Verificación del firmando


# ------------------------------
# Proceso del sellado de tiempo
tsr_filename="$1.tsr"

echo -e "\n \n---------------------- \nRealizando Sellado de Tiempo... "
sleep 1
openssl ts -query -data "$1" -sha512 -cert -no_nonce -out request.tsq
curl -H "Content-Type: application/timestamp-query" --data-binary '@request.tsq' http://tsa.sinpe.fi.cr/tsaHttp/ > "$tsr_filename"

sleep 1
echo -e "\n \nVerificando Estampado de Tiempo... \n"
sleep 2
openssl ts -reply -in "$tsr_filename" -text # Esta línea es de verificación


# ------------------------------
# Proceso de generación de sha1
sha1_filename="$1.sha1"

sleep 1
echo -e "\n \n---------------------- \nRealizando Hash de Sha1..."
sleep 1
sha1sum "$1" > "$sha1_filename"


echo -e "\nImpresión del Hash..."
sleep 1
cat "$sha1_filename"


# ------------------------------
# Procesos extra
rm request.tsq #Eliminación del request del timestamp



