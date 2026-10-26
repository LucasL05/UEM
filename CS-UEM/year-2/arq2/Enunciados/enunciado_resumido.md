# Trabalhos de Arquitetura de Computadores: extensões do gem5

## Finalidade

Este catálogo propõe **36 trabalhos práticos**, distribuídos pelos seis tópicos da disciplina. Cada proposta exige alterar o código-fonte do gem5 ou acrescentar um componente, uma política, uma métrica ou um mecanismo novo — não apenas executar configurações já existentes.

O público é formado por estudantes do segundo ano de Ciência da Computação que dominam Python, C e C++. Cada tema foi delimitado para ser realizável por uma dupla ou trio em aproximadamente **três meses**, incluindo estudo do código, implementação, testes, experimentos e relatório.

## Regras gerais sugeridas

- A entrega deve conter *fork* ou repositório com *commits*, instruções de compilação e execução, código documentado e pelo menos um script Python de configuração.
- O grupo deve criar um microbenchmark em C/C++ quando ele for necessário para evidenciar o mecanismo implementado.
- Toda proposta deve ser comparada contra uma **linha de base**: a versão original do gem5 ou uma política padrão equivalente.
- Os experimentos devem ser reprodutíveis: informar versão/commit do gem5, ISA, modelo de CPU, frequência, caches, memória, *workload*, número de repetições e comandos.
- O relatório deve separar corretamente: desempenho do programa simulado, estatísticas microarquiteturais e custo de simulação no hospedeiro.
- Recomenda-se começar em modo **SE** e com `build/X86/gem5.opt` ou `build/ARM/gem5.opt`. Trabalhos que usam Ruby ou GPU podem exigir uma configuração e compilação específicas.

## Critérios transversais de avaliação sugeridos

| Critério | Peso sugerido |
|---|---:|
| Correção e integração da funcionalidade ao gem5 | 35% |
| Qualidade de engenharia: organização, testes e documentação | 20% |
| Metodologia experimental e reprodutibilidade | 25% |
| Análise crítica dos resultados e apresentação | 20% |

---


## 8. Topologia *fat-tree* compacta

**Objetivo.** Explorar uma interconexão hierárquica com maior largura de banda próxima à raiz.

**O que implementar.** Adicionar uma topologia Ruby *fat-tree* para 4, 8 ou 16 controladores. Construir níveis de roteadores e enlaces com largura de banda ou número de enlaces maior nos níveis superiores. Documentar as restrições de número de nós aceitas.

**Como avaliar a implementação.** Comparar *fat-tree*, anel e mesh em tráfego uniforme e em um benchmark de compartilhamento de dados. Coletar latência de pacotes, utilização dos enlaces, contenção e desempenho. Avaliar se o benefício da árvore compensa seu maior número de enlaces.


# Orientação para escolha do tema

Os temas **1–6** tendem a exigir mais programação de benchmarks e instrumentação; os temas **7–12** concentram-se em Ruby/Garnet e topologias; os temas **13–18** exigem leitura mais cuidadosa de controladores e protocolos de coerência; os temas **19–24** exploram caches, memória e interferência em sistemas multinúcleo; os temas **25–30** podem ser realizados como extensões didáticas de uma ISA/modelo de execução vetorial; e os temas **31–36** dependem da infraestrutura GPU instalada e devem ser selecionados apenas se ela estiver funcional no ambiente da disciplina.

Para reduzir risco, o professor pode permitir que cada grupo entregue em três marcos:

1. **Semana 3–4:** desenho do mecanismo, leitura de código e experimento de linha de base;
2. **Semana 7–8:** versão funcional mínima, teste de correção e primeiras estatísticas;
3. **Semana 11–12:** campanha experimental, relatório e demonstração.

A comparação com a linha de base é obrigatória, mas não se espera que toda proposta gere melhoria de desempenho. Um resultado negativo é válido quando o mecanismo está correto, a metodologia é sólida e a análise explica as causas do comportamento observado.
