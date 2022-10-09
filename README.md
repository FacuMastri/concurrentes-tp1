# Trabajo Práctico 1 - Internet of Coffee

[Enunciado](https://concurrentes-fiuba.github.io/2C2022_tp1.html)

**Alumno**: Mastricchio, Facundo Rodrigo - **Padrón**: 100874

## Ejecución

La aplicación lee por `stdin` la ruta específica por donde tomar un archivo `.csv`que contenga los pedidos de bebidas.
El archivo en cuestión no debe tener headers, y el orden de las columnas debe ser el siguiente: `cantidad_cafe | cantidad_leche | cantidad_agua`.
Es necesario que el archivo siempre tenga la misma cantidad de columnas, y que cada una de ellas contenga un número entero positivo.
Por ejemplo, (0,1,2), (1,2,3), (1,0,1), (0,0,1), (1,0,0), (0,1,0) son líneas válidas; mientras que la línea (0,0,0) se encuentra
reservada para indicar el fin de los pedidos.

    cargo run < <ruta-pedidos-csv>

Por ejemplo:

    cargo run < src/orders/orders.csv

## Hipótesis y supuestos

Las hipótesis y supuestos tomados para el desarrollo del presente trabajo práctico fueron:

- 

## Detalles de implementación

## Cuestiones a mejorar
