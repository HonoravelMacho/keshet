# קֶשֶׁת - KESHET v1.0.2 🏹
### *Symbolic Brute-Force & Universal Identity Hunter*

**Keshet** (do hebraico: Arco/Arco-íris) é um motor de busca analítica de alta performance desenvolvido em **Rust**. Projetado para matemáticos experimentais e entusiastas da teoria dos números, o Keshet vasculha trilhões de combinações matemáticas em busca de identidades transcendentais e polinômios geratrizes para constantes irracionais.

---

## 🚀 Diferenciais da Sonda

- **Quantum Cache (RAM Memoization):** Utiliza até 1/3 da memória livre para pré-calcular termos complexos, acelerando a busca em ordens de magnitude.
- **Identidade Rabeliana 3.0:** Motor exclusivo que busca unificação entre estruturas exponenciais e séries de potências (Zeta).
- **Aritmética de 128-bits:** Precisão científica garantida pela biblioteca GMP (`rug`), permitindo validações com mais de 30 casas decimais.
- **Protocolo de Integridade (Anti-Preguiça):** Filtros inteligentes que banem identidades triviais ($\ln 1$, $\sqrt{4}$, $\pi/\pi$) para entregar apenas "Ouro Puro".
- **Visual Didático:** Renderização de fórmulas em estilo de livro diretamente no terminal.

---

## 🛠 Instalação

### 💻 No Linux (Pop!_OS / Ubuntu / Debian)
```bash
# Instalar dependências do sistema
sudo apt update && sudo apt install build-essential libgmp-dev -y

# Clonar e Compilar
git clone https://github.com/HonoravelMacho/keshet.git
cd keshet
cargo build --release

# Instalar globalmente
sudo install -m 755 target/release/keshet /usr/local/bin/keshet


###📱 No Termux (Android)
# Atualizar repositórios e instalar ferramentas básicas
pkg update && pkg upgrade
pkg install git rust clang make libgmp -y

# Clonar e Compilar
git clone https://github.com/HonoravelMacho/keshet.git
cd keshet
cargo build --release

# Configurar o Alias para abrir apenas digitando 'keshet'
echo "alias keshet='$HOME/keshet/target/release/keshet'" >> ~/.zshrc
source ~/.zshrc

###⚓ Comando de Alias (Acesso Rápido)

Para rodar o Keshet de qualquer pasta sem precisar digitar o caminho completo no Linux:
Para ZSH (Padrão no Pop!_OS):
echo "alias keshet='/usr/local/bin/keshet'" >> ~/.zshrc && source ~/.zshrc

###📱 No Termux (Android)
# Atualizar repositórios e instalar ferramentas básicas
pkg update && pkg upgrade
pkg install git rust clang make libgmp -y

# Clonar e Compilar
git clone https://github.com/HonoravelMacho/keshet.git
cd keshet
cargo build --release

# Configurar o Alias para abrir apenas digitando 'keshet'
echo "alias keshet='$HOME/keshet/target/release/keshet'" >> ~/.zshrc
source ~/.zshrc

###⚓ Comando de Alias (Acesso Rápido)
Para rodar o Keshet de qualquer pasta sem precisar digitar o caminho completo no Linux:
Para ZSH (Padrão no Pop!_OS):
echo "alias keshet='/usr/local/bin/keshet'" >> ~/.zshrc && source ~/.zshrc

###🔄 Como Atualizar
Sempre que uma nova versão for lançada, atualize seu navio rapidamente:
cd ~/keshet
git pull
cargo build --release
# Se estiver no Linux, reinstale o binário:
sudo install -m 755 target/release/keshet /usr/local/bin/keshet

