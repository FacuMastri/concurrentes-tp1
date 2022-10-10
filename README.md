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

- La capacidad de los contenedores de granos de café para moler y de leche fría para convertir en leche espumada debe ser suficiente (configurable) para satisfacer todos los pedidos.
- Cuando no se tiene los recursos necesarios para satisfacer un pedido, el correspondiente contenedor de granos de café o de leche fría reabastece los contenedores principales
con un factor de reabastecimiento configurable para no tener que reabastece cada vez que se necesite un recurso. Esto confronta de alguna manera con el requerimiento de
"_Cuando el contenedor de cafe molido se agota, el molinillo automático toma una cantidad de granos y los convierte en café molido._", 
ya que no se espera a que se agote un recurso, pero es mejor que descartar un pedido.
- El orden de aplicación de los recursos es el siguiente: `café molido -> leche espumada -> agua`, si corresponde.
- Se aceptan pedidos de solo leche, solo café, solo agua, o de cualquier combinación de los mismos. Esto es para mostrar que, por ejemplo,
si se está utilizando el café molido en algún pedido, todavía es posible completar pedidos de solo leche o solo agua (o una combinación de ambas).
- El tiempo de preparación de una bebida es la suma del tiempo de procesamiento de cada recurso involucrado en la elaboración de la misma. Dicho
tiempo de procesamiento está afectado por un factor de procesamiento (configurable) y por la cantidad de recursos que se estén utilizando en el momento.
- No se lleva la estadística del agua utilizada o consumida, ya que se entiende que para el negocio (inclusive a nivel de costo) tiene mayor impacto el café y la leche utilizados.


## Detalles de implementación

### Crates utilizados

## Cuestiones a mejorar
