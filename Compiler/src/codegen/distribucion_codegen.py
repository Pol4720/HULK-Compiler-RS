import os
import re

# Configuración
TOKENS_DIR = os.path.join("Compiler", "src", "hulk_Tokens")
IMPLEMENTATION_PATTERN = r"impl\s+CodegenNode\s+for\s+(\w+)"
NODOS_DIFICULTAD = {
    # nodo: dificultad (1: fácil, 2: media, 3: difícil)
    "hulk_binary_expr.rs": 2,
    "hulk_unary_expr.rs": 2,
    "hulk_literal.rs": 1,
    "hulk_identifier.rs": 1,
    "hulk_assignment.rs": 2,
    "hulk_if_expr.rs": 3,
    "hulk_while_loop.rs": 3,
    "hulk_for_loop.rs": 3,
    "hulk_function.rs": 3,
    "hulk_call.rs": 2,
    "hulk_block.rs": 2,
    "hulk_return.rs": 2,
    "hulk_program.rs": 1,
    "hulk_print.rs": 1,
    "hulk_expression.rs": 2,
    "hulk_keywords.rs": 1,
    "hulk_for_expr.rs": 3,
    "hulk_ifExp.rs": 3,
    "hulk_function_info.rs": 2,
    "hulk_destructive_assign.rs": 2,
    "hulk_whileloop.rs": 3,
    "hulk_let_in.rs": 2,
    "hulk_program.rs": 1,
    "hulk_code_block.rs": 2,
    "hulk_function_def.rs": 3,
    "hulk_function_call.rs": 2,
    "hulk_assignment.rs": 2,
    "hulk_binary_expr.rs": 2,
    "hulk_unary_expr.rs": 2,
    "hulk_operators.rs": 2,
    "hulk_literal.rs": 1,
    "hulk_identifier.rs": 1,
}

# Similitudes manuales (pares de nodos similares)
SIMILARES = [
    ("hulk_binary_expr.rs", "hulk_unary_expr.rs"),
    ("hulk_if_expr.rs", "hulk_ifExp.rs"),
    ("hulk_while_loop.rs", "hulk_whileloop.rs"),
    ("hulk_for_loop.rs", "hulk_for_expr.rs"),
    ("hulk_assignment.rs", "hulk_destructive_assign.rs"),
    ("hulk_function.rs", "hulk_function_def.rs"),
    ("hulk_call.rs", "hulk_function_call.rs"),
    ("hulk_block.rs", "hulk_code_block.rs"),
    ("hulk_program.rs", "hulk_program.rs"),  # self-pair, for completeness
    ("hulk_literal.rs", "hulk_identifier.rs"),
    ("hulk_print.rs", "hulk_keywords.rs"),
    ("hulk_expression.rs", "hulk_operators.rs"),
    ("hulk_let_in.rs", "hulk_return.rs"),
    ("hulk_function_info.rs", "hulk_keywords.rs"),
]

PROGRAMADORES = ["Richard", "Abraham"]

def listar_nodos():
    nodos = []
    for fname in os.listdir(TOKENS_DIR):
        if fname.endswith(".rs") and fname.startswith("hulk_"):
            nodos.append(fname)
    # Si hay nodos en dificultad y no en carpeta, los agregamos igual
    for k in NODOS_DIFICULTAD:
        if k not in nodos:
            nodos.append(k)
    return sorted(nodos)

def buscar_implementaciones():
    implementados = set()
    for fname in os.listdir(TOKENS_DIR):
        if fname.endswith(".rs"):
            path = os.path.join(TOKENS_DIR, fname)
            with open(path, encoding="utf8") as f:
                contenido = f.read()
                for m in re.finditer(IMPLEMENTATION_PATTERN, contenido):
                    implementados.add(m.group(1))
    return implementados

def nodo_a_struct(nodo):
    # Convierte hulk_binary_expr.rs -> BinaryExpr
    base = nodo.replace("hulk_", "").replace(".rs", "")
    partes = base.split("_")
    return "".join([p.capitalize() for p in partes]) + ("Expr" if "expr" in base and not base.endswith("expr") else "")

def distribuir_nodos(nodos):
    # Emparejar similares primero
    asignacion = {p: [] for p in PROGRAMADORES}
    asignados = set()
    dificultad = {n: NODOS_DIFICULTAD.get(n, 2) for n in nodos}

    # Emparejar similares
    for a, b in SIMILARES:
        if a in nodos and b in nodos and a not in asignados and b not in asignados:
            # Alternar asignación
            if len(asignacion[PROGRAMADORES[0]]) <= len(asignacion[PROGRAMADORES[1]]):
                asignacion[PROGRAMADORES[0]].append(a)
                asignacion[PROGRAMADORES[1]].append(b)
            else:
                asignacion[PROGRAMADORES[1]].append(a)
                asignacion[PROGRAMADORES[0]].append(b)
            asignados.add(a)
            asignados.add(b)

    # Asignar el resto por dificultad para balancear
    restantes = [n for n in nodos if n not in asignados]
    restantes.sort(key=lambda n: -dificultad[n])  # primero los difíciles

    suma = {p: sum(dificultad[n] for n in asignacion[p]) for p in PROGRAMADORES}
    for n in restantes:
        p = min(PROGRAMADORES, key=lambda x: suma[x])
        asignacion[p].append(n)
        suma[p] += dificultad[n]
    return asignacion

def main():
    nodos = listar_nodos()
    implementados = buscar_implementaciones()
    asignacion = distribuir_nodos(nodos)

    print("=== Distribución de nodos para CodegenNode ===\n")
    # For Windows console compatibility, replace unicode with ASCII equivalents
    check = "[OK]"
    cross = "[NO]"
    for p in PROGRAMADORES:
        print(f"\n{p}:")
        for nodo in asignacion[p]:
            struct = nodo_a_struct(nodo)
            hecho = check if struct in implementados else cross
            print(f"  - {nodo:<22} ({struct:<18}) {hecho}")
    print("\nLeyenda: [OK] Implementado   [NO] Pendiente")

if __name__ == "__main__":
    main()
