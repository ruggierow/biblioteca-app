# /corrigir — Protocolo de Correção Segura

Use este protocolo SEMPRE antes de implementar qualquer correção de bug.

## Passos obrigatórios

### 1. Diagnóstico
- Identifique a causa raiz exata do problema
- Indique o arquivo e número de linha onde a correção deve ser feita

### 2. Escopo mínimo
- Liste explicitamente quais linhas/funções SERÃO alteradas
- Liste o que NÃO será tocado, especialmente código adjacente ao problema
- Se a correção exigir alterar mais de um trecho, justifique cada um

### 3. Proposta (PARAR e aguardar aprovação)
- Mostre o diff planejado (antes → depois) para cada alteração
- Aguarde o usuário aprovar antes de implementar

### 4. Implementação
- Faça somente as alterações aprovadas no passo 3
- Não aproveite para "limpar", "refatorar" ou "melhorar" código não relacionado

### 5. Verificação pós-edição
- Confirme que nenhuma linha fora do escopo aprovado foi alterada
- Se houver mudança não planejada, reverta e explique o que aconteceu

## Regra fundamental

> Corrigir um bug não justifica remover ou alterar código que não tem relação direta com o problema. Em caso de dúvida, não altere.
