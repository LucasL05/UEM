---
marp: true
theme: uncover
---

# Hoje  não teremos teoria
Abram o Gemini ou o ChatGPT no navegador de vocês

---

### O que vamos fazer na aula de hoje?
- Criar um site funcional personalizado do zero com IA;
- Salvar nosso progresso no GitHub.

---

# *Vamos começar!*

---

<style scoped>
section {
  justify-content: flex-start !important;
  padding-top: 0px !important; /* controla o quão perto do topo absoluto ela fica */
}

/* Tamanho da fonte */
p, ul, li {
  font-size: 0.8rem;
}

/* Espaço extra abaixo do primeiro item */
li:first-child {
  margin-bottom: rem;
}

/* Segundo item da lista */
li:nth-child(2) {
  margin-bottom: 2rem;
}

/* Terceiro item da lista */
li:nth-child(3) {
  margin-bottom: rem;
}

</style>

- **Nosso primeiro passo será pedir para a IA criar o template do nosso site.**

- **Sejam criativos nas especificações!**

- **Após isso, nós teremos o código em mãos...**

- **Mas como transformá-lo em um site?**

![bg right:50% contain](./p1.png)

---

# Passo 2
### Criando um template (esqueleto)

---

<style scoped>
section {
  position: relative;
  justify-content: flex-start !important;
  padding-bottom: 35% !important; /* Cria um respiro para o texto nunca bater na imagem */
}

.bottom-banner {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 50%;
  object-fit: fill; /* Preenche a largura sem esticar ou deformar */
}

p, ul, li {
  font-size: 1.2rem;
}

li:first-child {
  margin-bottom: 1rem;
}

/* Segundo item da lista */
li:nth-child(2) {
  margin-bottom: 0.5rem;
}

/* Terceiro item da lista */
li:nth-child(3) {
  margin-bottom: 0.5rem;
}

</style>


- **abra o Terminal do seu computador;**
- **Digite o comando abaixo e selecione "yes";**

<img src="./t2.png" class="bottom-banner" />

---

<style scoped>
section {
  position: relative;
  justify-content: flex-start !important;
  padding-bottom: 35% !important; /* Cria um respiro para o texto nunca bater na imagem */
}

.bottom-banner {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 48%;
  object-fit: fill; /* Preenche a largura sem esticar ou deformar */
}

p, ul, li {
  font-size: 0.9rem;
}

li:first-child {
  margin-bottom: 1rem;
}

/* Segundo item da lista */
li:nth-child(2) {
  margin-bottom: 0.2rem;
}

/* Terceiro item da lista */
li:nth-child(3) {
  margin-bottom: 0.5rem;
}

</style>

- **O  que esse npm bla bla bla quer dizer?**
- **O npm é um gerenciador de arquivos;**
- **nesse comando, nós estamos pedindo para ele criar um: template (modelo) vanilla (padrão) para o nosso site feito na linguagem TypeScript, por isso o -ts no final;**


<img src="./t2.png" class="bottom-banner" />

---

<style scoped>
section {
  display: flex !important;
  flex-direction: column !important;
  justify-content: flex-start !important;
  position: relative;
  padding: 40px;
}

ul {
  margin: 0 !important;
  padding-left: 24px !important;
}

li {
  font-size: 1.2rem;
  color: #333;
}

/* Fixa o terminal no canto inferior ocupando a largura útil */
.terminal-box {
  position: absolute;
  bottom: 30px;
  left: 40px;
  right: 40px;
  margin: 0 !important;
  background-color: #1e1e1e;
  border-radius: 6px;
}

.terminal-box code {
  display: block;
  font-size: 0.65rem;
  line-height: 1.3;
  padding: 16px;
  color: #d4d4d4;
  background: transparent !important;
}

p, ul, li {
  font-size: 0.9rem;
}

li:first-child {
  margin-bottom: 0.5rem;
}

/* Segundo item da lista */
li:nth-child(2) {
  margin-bottom: 0.2rem;
}

/* Terceiro item da lista */
li:nth-child(3) {
  margin-bottom: 0.5rem;
}

</style>

- **Ele gerou um link local!**
- **Se você copiar e colar ele no navegador, vai ver que agora temos um site no ar.**
- **Mas por enquanto esse é só o modelo genérico do npm...**

<pre class="terminal-box"><code>◇  Starting dev server...

> nome-do-seu-site@0.0.0 dev
> vite

  VITE v8.3.1  ready in 560 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help</code></pre>

  ---
  # Passo 3
  ### Personalizando nosso template (esqueleto)
  ---

  