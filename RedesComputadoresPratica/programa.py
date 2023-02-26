comprimento = 1.75
largura = 1.75
area_peca = comprimento * largura
area_total = 378.54
area_inutil = 0
area_util = area_total - area_inutil
area_divisao_exata = 0
numero_pecas = 0

if int(area_util) % area_peca == 0:
    numero_pecas = area_util/area_peca
else: #area nao é multiplo exato
    area_divisao_exata = area_util - (area_util % area_peca) #area util - area que sobrou
    precas_util = area_divisao_exata / area_peca            # numero de peças da area de divisao exata
    area_sobra = area_util % area_peca
    if precas_util < 10:
         numero_pecas = precas_util + 2
    else:
        numero_pecas = precas_util + (0.17 * precas_util)

print(f"Piso: {comprimento} X {largura} : número de pisos: {int(numero_pecas)}")
