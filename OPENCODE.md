# OpenCode - Modelos Gratuitos

Este projeto está configurado para usar OpenCode com modelos gratuitos.

## Modelos Disponíveis

| Modelo | Comando |
|--------|---------|
| MiniMax M2.5 Free | `opencode -m opencode/minimax-m2.5-free .` |
| Big Pickle | `opencode -m opencode/big-pickle .` |
| Trinity Large | `opencode -m opencode/trinity-large-preview-free .` |
| GPT-5 Nano | `opencode -m opencode/gpt-5-nano .` |

## Como Usar

### MiniMax M2.5 (Recomendado para código)
```bash
opencode -m opencode/minimax-m2.5-free .
```

### Big Pickle
```bash
opencode -m opencode/big-pickle .
```

## Skills Disponíveis

As seguintes skills já estão configuradas e funcionam com OpenCode:

- **prd** - Gerar Documento de Requisitos de Produto
- **ralph** - Converter PRD para formato JSON

## Dica

Para uma experiência otimizada, adicione um alias no seu ~/.zshrc:

```bash
alias ocode-minimax='opencode -m opencode/minimax-m2.5-free'
alias ocode-pickle='opencode -m opencode/big-pickle'
```

Agora basta rodar:
```bash
ocode-minimax
```
