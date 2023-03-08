lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Seu Computador"),
        ("desk_tip", "Seu computador pode ser acessado com este ID e senha."),
        ("Password", "Senha"),
        ("Ready", "Pronto"),
        ("Established", "Estabelecido"),
        ("connecting_status", "Conectando à rede do RustDesk..."),
        ("Enable Service", "Habilitar Serviço"),
        ("Start Service", "Iniciar Serviço"),
        ("Service is running", "Serviço está em execução"),
        ("Service is not running", "Serviço não está em execução"),
        ("not_ready_status", "Não está pronto. Por favor verifique sua conexão"),
        ("Control Remote Desktop", "Controle um Computador Remoto"),
        ("Transfer File", "Transferir Arquivo"),
        ("Connect", "Conectar"),
        ("Recent Sessions", "Sessões Recentes"),
        ("Address Book", "Lista de Endereços"),
        ("Confirmation", "Confirmação"),
        ("TCP Tunneling", "Tunelamento TCP"),
        ("Remove", "Remover"),
        ("Refresh random password", "Atualizar senha aleatória"),
        ("Set your own password", "Configure sua própria senha"),
        ("Enable Keyboard/Mouse", "Habilitar teclado/mouse"),
        ("Enable Clipboard", "Habilitar Área de Transferência"),
        ("Enable File Transfer", "Habilitar Transferência de Arquivos"),
        ("Enable TCP Tunneling", "Habilitar Tunelamento TCP"),
        ("IP Whitelisting", "Lista de IPs Confiáveis"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import Server Config", "Importar Configuração do Servidor"),
        ("Export Server Config", "Exportar Configuração do Servidor"),
        ("Import server configuration successfully", "Configuração do servidor importada com sucesso"),
        ("Export server configuration successfully", "Configuração do servidor exportada com sucesso"),
        ("Invalid server configuration", "Configuração do servidor inválida"),
        ("Clipboard is empty", "A área de transferência está vazia"),
        ("Stop service", "Parar serviço"),
        ("Change ID", "Alterar ID"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Somente os caracteres a-z, A-Z, 0-9 e _ (sublinhado) são permitidos. A primeira letra deve ser a-z, A-Z. Comprimento entre 6 e 16."),
        ("Website", "Website"),
        ("About", "Sobre"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Desativar som"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Entrada de Áudio"),
        ("Enhancements", "Melhorias"),
        ("Hardware Codec", "Codec de hardware"),
        ("Adaptive Bitrate", "Taxa de bits adaptável"),
        ("ID Server", "Servidor de ID"),
        ("Relay Server", "Servidor de Relay"),
        ("API Server", "Servidor da API"),
        ("invalid_http", "deve iniciar com http:// ou https://"),
        ("Invalid IP", "IP inválido"),
        ("Invalid format", "Formato inválido"),
        ("server_not_support", "Ainda não suportado pelo servidor"),
        ("Not available", "Indisponível"),
        ("Too frequent", "Muito frequente"),
        ("Cancel", "Cancelar"),
        ("Skip", "Pular"),
        ("Close", "Fechar"),
        ("Retry", "Tentar novamente"),
        ("OK", "OK"),
        ("Password Required", "Senha necessária"),
        ("Please enter your password", "Por favor informe sua senha"),
        ("Remember password", "Lembrar senha"),
        ("Wrong Password", "Senha incorreta"),
        ("Do you want to enter again?", "Você deseja conectar novamente?"),
        ("Connection Error", "Erro de conexão"),
        ("Error", "Erro"),
        ("Reset by the peer", "Reiniciado pelo parceiro"),
        ("Connecting...", "Conectando..."),
        ("Connection in progress. Please wait.", "Conexão em progresso. Aguarde por favor."),
        ("Please try 1 minute later", "Por favor tente após 1 minuto"),
        ("Login Error", "Erro de login"),
        ("Successful", "Sucesso"),
        ("Connected, waiting for image...", "Conectado. Aguardando pela imagem..."),
        ("Name", "Nome"),
        ("Type", "Tipo"),
        ("Modified", "Modificado"),
        ("Size", "Tamanho"),
        ("Show Hidden Files", "Mostrar Arquivos Ocultos"),
        ("Receive", "Receber"),
        ("Send", "Enviar"),
        ("Refresh File", "Atualizar Arquivo"),
        ("Local", "Local"),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computador Remoto"),
        ("Local Computer", "Computador Local"),
        ("Confirm Delete", "Confirmar Apagar"),
        ("Delete", "Apagar"),
        ("Properties", "Propriedades"),
        ("Multi Select", "Seleção múltipla"),
        ("Select All", "Selecionar tudo"),
        ("Unselect All", "Desmarcar tudo"),
        ("Empty Directory", "Diretório vazio"),
        ("Not an empty directory", "Diretório não está vazio"),
        ("Are you sure you want to delete this file?", "Tem certeza que deseja apagar este arquivo?"),
        ("Are you sure you want to delete this empty directory?", "Tem certeza que deseja apagar este diretório vazio?"),
        ("Are you sure you want to delete the file of this directory?", "Tem certeza que deseja apagar este arquivo deste diretório?"),
        ("Do this for all conflicts", "Fazer isto para todos os conflitos"),
        ("This is irreversible!", "Isso é irreversível!"),
        ("Deleting", "Apagando"),
        ("files", "arquivos"),
        ("Waiting", "Aguardando"),
        ("Finished", "Completo"),
        ("Speed", "Velocidade"),
        ("Custom Image Quality", "Qualidade Visual Personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de usuário"),
        ("Unblock user input", "Desbloquear entrada de usuário"),
        ("Adjust Window", "Ajustar Janela"),
        ("Original", "Original"),
        ("Shrink", "Reduzir"),
        ("Stretch", "Aumentar"),
        ("Scrollbar", "Barra de rolagem"),
        ("ScrollAuto", "Rolagem automática"),
        ("Good image quality", "Qualidade visual boa"),
        ("Balanced", "Balanceada"),
        ("Optimize reaction time", "Otimizar tempo de reação"),
        ("Custom", "Personalizado"),
        ("Show remote cursor", "Mostrar cursor remoto"),
        ("Show quality monitor", "Exibir monitor de qualidade"),
        ("Disable clipboard", "Desabilitar área de transferência"),
        ("Lock after session end", "Bloquear após o fim da sessão"),
        ("Insert", "Inserir"),
        ("Insert Lock", "Bloquear computador"),
        ("Refresh", "Atualizar"),
        ("ID does not exist", "ID não existe"),
        ("Failed to connect to rendezvous server", "Falha ao conectar ao servidor de rendezvous"),
        ("Please try later", "Por favor tente mais tarde"),
        ("Remote desktop is offline", "O computador remoto está offline"),
        ("Key mismatch", "Chaves incompatíveis"),
        ("Timeout", "Tempo esgotado"),
        ("Failed to connect to relay server", "Falha ao conectar ao servidor de relay"),
        ("Failed to connect via rendezvous server", "Falha ao conectar ao servidor de rendezvous"),
        ("Failed to connect via relay server", "Falha ao conectar através do servidor de relay"),
        ("Failed to make direct connection to remote desktop", "Falha ao fazer conexão direta ao desktop remoto"),
        ("Set Password", "Definir Senha"),
        ("OS Password", "Senha do SO"),
        ("install_tip", "Devido ao UAC, o RustDesk não funciona corretamente como o lado remoto em alguns casos. Para evitar o UAC, por favor clique no botão abaixo para instalar o RustDesk no sistema."),
        ("Click to upgrade", "Clique para fazer o upgrade"),
        ("Click to download", "Clique para baixar"),
        ("Click to update", "Clique para fazer o update"),
        ("Configure", "Configurar"),
        ("config_acc", "Para controlar seu computador remotamente, você precisa conceder ao RustDesk permissões de \"Acessibilidade\"."),
        ("config_screen", "Para acessar seu computador remotamente, você precisa conceder ao RustDesk permissões de \"Gravar a Tela\"/"),
        ("Installing ...", "Instalando ..."),
        ("Install", "Instalar"),
        ("Installation", "Instalação"),
        ("Installation Path", "Caminho da Instalação"),
        ("Create start menu shortcuts", "Criar atalhos no Menu Iniciar"),
        ("Create desktop icon", "Criar ícone na Área de Trabalho"),
        ("agreement_tip", "Ao iniciar a instalação, você concorda com o acordo de licença."),
        ("Accept and Install", "Aceitar e Instalar"),
        ("End-user license agreement", "Acordo de licença do usuário final"),
        ("Generating ...", "Gerando ..."),
        ("Your installation is lower version.", "Sua instalação é de uma versão menor."),
        ("not_close_tcp_tip", "Não feche esta janela enquanto estiver utilizando o túnel"),
        ("Listening ...", "Escutando ..."),
        ("Remote Host", "Host Remoto"),
        ("Remote Port", "Porta Remota"),
        ("Action", "Ação"),
        ("Add", "Adicionar"),
        ("Local Port", "Porta Local"),
        ("Local Address", "Endereço Local"),
        ("Change Local Port", "Alterar Porta Local"),
        ("setup_server_tip", "Para uma conexão mais rápida, por favor configure seu próprio servidor"),
        ("Too short, at least 6 characters.", "Muito curto, pelo menos 6 caracteres."),
        ("The confirmation is not identical.", "A confirmação não é idêntica."),
        ("Permissions", "Permissões"),
        ("Accept", "Aceitar"),
        ("Dismiss", "Dispensar"),
        ("Disconnect", "Desconectar"),
        ("Allow using keyboard and mouse", "Permitir o uso de teclado e mouse"),
        ("Allow using clipboard", "Permitir o uso da área de transferência"),
        ("Allow hearing sound", "Permitir escutar som"),
        ("Allow file copy and paste", "Permitir copiar e colar arquivos"),
        ("Connected", "Conectado"),
        ("Direct and encrypted connection", "Conexão direta e criptografada"),
        ("Relayed and encrypted connection", "Conexão via relay e criptografada"),
        ("Direct and unencrypted connection", "Conexão direta e não criptografada"),
        ("Relayed and unencrypted connection", "Conexão via relay e não criptografada"),
        ("Enter Remote ID", "Informe o ID Remoto"),
        ("Enter your password", "Informe sua senha"),
        ("Logging in...", "Fazendo Login..."),
        ("Enable RDP session sharing", "Habilitar compartilhamento de sessão RDP"),
        ("Auto Login", "Login Automático (Somente válido se você habilitou \"Bloquear após o fim da sessão\")"),
        ("Enable Direct IP Access", "Habilitar Acesso IP Direto"),
        ("Rename", "Renomear"),
        ("Space", "Espaço"),
        ("Create Desktop Shortcut", "Criar Atalho na Área de Trabalho"),
        ("Change Path", "Alterar Caminho"),
        ("Create Folder", "Criar Diretório"),
        ("Please enter the folder name", "Por favor informe o nome do diretório"),
        ("Fix it", "Corrigir"),
        ("Warning", "Aviso"),
        ("Login screen using Wayland is not supported", "Tela de Login utilizando Wayland não é suportada"),
        ("Reboot required", "Reinicialização necessária"),
        ("Unsupported display server", "Servidor de display não suportado"),
        ("x11 expected", "x11 esperado"),
        ("Port", "Porta"),
        ("Settings", "Configurações"),
        ("Username", "Nome de usuário"),
        ("Invalid port", "Porta inválida"),
        ("Closed manually by the peer", "Fechada manualmente pelo parceiro"),
        ("Enable remote configuration modification", "Habilitar modificações de configuração remotas"),
        ("Run without install", "Executar sem instalar"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Sempre conectar via relay"),
        ("whitelist_tip", "Somente IPs confiáveis podem me acessar"),
        ("Login", "Login"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Sair"),
        ("Tags", "Tags"),
        ("Search ID", "Pesquisar ID"),
        ("whitelist_sep", "Separado por vírcula, ponto-e-vírgula, espaços ou nova linha"),
        ("Add ID", "Adicionar ID"),
        ("Add Tag", "Adicionar Tag"),
        ("Unselect all tags", "Desmarcar todas as tags"),
        ("Network error", "Erro de rede"),
        ("Username missed", "Nome de usuário requerido"),
        ("Password missed", "Senha requerida"),
        ("Wrong credentials", "Nome de usuário ou senha incorretos"),
        ("Edit Tag", "Editar Tag"),
        ("Unremember Password", "Esquecer Senha"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Adicionar aos Favoritos"),
        ("Remove from Favorites", "Remover dos Favoritos"),
        ("Empty", "Vazio"),
        ("Invalid folder name", "Nome de diretório inválido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nome de anfitrião"),
        ("Discovered", "Descoberto"),
        ("install_daemon_tip", "Para inicialização junto ao sistema, você deve instalar o serviço de sistema."),
        ("Remote ID", "ID Remoto"),
        ("Paste", "Colar"),
        ("Paste here?", "Colar aqui?"),
        ("Are you sure to close the connection?", "Tem certeza que deseja fechar a conexão?"),
        ("Download new version", "Baixar nova versão"),
        ("Touch mode", "Modo toque"),
        ("Mouse mode", "Modo mouse"),
        ("One-Finger Tap", "Toque com um dedo"),
        ("Left Mouse", "Botão esquerdo do mouse"),
        ("One-Long Tap", "Um toque longo"),
        ("Two-Finger Tap", "Toque com dois dedos"),
        ("Right Mouse", "Botão direito do mouse"),
        ("One-Finger Move", "Mover com um dedo"),
        ("Double Tap & Move", "Toque duplo & mover"),
        ("Mouse Drag", "Arrastar com o mouse"),
        ("Three-Finger vertically", "Três dedos verticalmente"),
        ("Mouse Wheel", "Roda do Mouse"),
        ("Two-Finger Move", "Mover com dois dedos"),
        ("Canvas Move", "Mover Tela"),
        ("Pinch to Zoom", "Pinçar para Zoom"),
        ("Canvas Zoom", "Zoom na tela"),
        ("Reset canvas", "Reiniciar tela"),
        ("No permission of file transfer", "Sem permissão para transferência de arquivo"),
        ("Note", "Nota"),
        ("Connection", "Conexão"),
        ("Share Screen", "Compartilhar Tela"),
        ("CLOSE", "FECHAR"),
        ("OPEN", "ABRIR"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "itens"),
        ("Selected", "Selecionado"),
        ("Screen Capture", "Captura de Tela"),
        ("Input Control", "Controle de Entrada"),
        ("Audio Capture", "Captura de Áudio"),
        ("File Connection", "Conexão de Arquivo"),
        ("Screen Connection", "Conexão de Tela"),
        ("Do you accept?", "Você aceita?"),
        ("Open System Setting", "Abrir Configurações do Sistema"),
        ("How to get Android input permission?", "Como habilitar a permissão de entrada do Android?"),
        ("android_input_permission_tip1", "Para que um dispositivo remoto controle seu dispositivo Android via mouse ou toque, você precisa permitir que o RustDesk use o serviço \"Acessibilidade\"."),
        ("android_input_permission_tip2", "Por favor vá para a próxima página de configuração do sistema, encontre e entre [Serviços Instalados], HABILITE o serviço [RustDesk Input]."),
        ("android_new_connection_tip", "Nova requisição de controle recebida, solicita o controle de seu dispositivo atual."),
        ("android_service_will_start_tip", "Habilitar a Captura de Tela irá automaticamente inicalizar o serviço, permitindo que outros dispositivos solicitem uma conexão deste dispositivo."),
        ("android_stop_service_tip", "Fechar o serviço irá automaticamente fechar todas as conexões estabelecidas."),
        ("android_version_audio_tip", "A versão atual do Android não suporta captura de áudio, por favor atualize para o Android 10 ou superior."),
        ("android_start_service_tip", "Toque [Iniciar Serviço] ou abra a permissão [Captura de Tela] para iniciar o serviço de compartilhamento de tela."),
        ("Account", "Conta"),
        ("Overwrite", "Substituir"),
        ("This file exists, skip or overwrite this file?", "Este arquivo existe, pular ou substituir este arquivo?"),
        ("Quit", "Sair"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Ajuda"),
        ("Failed", "Falhou"),
        ("Succeeded", "Sucesso"),
        ("Someone turns on privacy mode, exit", "Alguém habilitou o modo de privacidade, sair"),
        ("Unsupported", "Não suportado"),
        ("Peer denied", "Parceiro negou"),
        ("Please install plugins", "Por favor instale plugins"),
        ("Peer exit", "Parceiro saiu"),
        ("Failed to turn off", "Falha ao desligar"),
        ("Turned off", "Desligado"),
        ("In privacy mode", "No modo de privacidade"),
        ("Out privacy mode", "Fora do modo de privacidade"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Manter o serviço do RustDesk executando em segundo plano"),
        ("Ignore Battery Optimizations", "Ignorar otimizações de bateria"),
        ("android_open_battery_optimizations_tip", "Abrir otimizações de bateria"),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Conexão não permitida"),
        ("Legacy mode", "Modo legado"),
        ("Map mode", "Modo mapa"),
        ("Translate mode", "Modo traduzido"),
        ("Use permanent password", "Utilizar senha permanente"),
        ("Use both passwords", "Utilizar ambas as senhas"),
        ("Set permanent password", "Configurar senha permanente"),
        ("Enable Remote Restart", "Habilitar reinicialização remota"),
        ("Allow remote restart", "Permitir reinicialização remota"),
        ("Restart Remote Device", "Reiniciar dispositivo remoto"),
        ("Are you sure you want to restart", "Você tem certeza que deseja reiniciar?"),
        ("Restarting Remote Device", "Reiniciando dispositivo remoto"),
        ("remote_restarting_tip", ""),
        ("Copied", "Copiado"),
        ("Exit Fullscreen", "Sair da Tela Cheia"),
        ("Fullscreen", "Tela Cheia"),
        ("Mobile Actions", "Ações móveis"),
        ("Select Monitor", "Selecionar monitor"),
        ("Control Actions", "Controlar ações"),
        ("Display Settings", "Configurações de exibição"),
        ("Ratio", "Proporção"),
        ("Image Quality", "Qualidade de imagem"),
        ("Scroll Style", "Estilo de rolagem"),
        ("Show Menubar", "Exibir barra de menu"),
        ("Hide Menubar", "Ocultar barra de menu"),
        ("Direct Connection", "Conexão direta"),
        ("Relay Connection", "Conexão relay"),
        ("Secure Connection", "Conexão segura"),
        ("Insecure Connection", "Conexão insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptada"),
        ("General", "Geral"),
        ("Security", "Segurança"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema escuro"),
        ("Light Theme", ""),
        ("Dark", "Escuro"),
        ("Light", "Claro"),
        ("Follow System", "Seguir sistema"),
        ("Enable hardware codec", "Habilitar codec de hardware"),
        ("Unlock Security Settings", "Desabilitar configurações de segurança"),
        ("Enable Audio", "Habilitar áudio"),
        ("Unlock Network Settings", "Desbloquear configurações de rede"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Acesso direto por IP"),
        ("Proxy", "Proxy"),
        ("Apply", "Aplicar"),
        ("Disconnect all devices?", "Desconectar todos os dispositivos?"),
        ("Clear", "Limpar"),
        ("Audio Input Device", "Dispositivo de entrada de áudio"),
        ("Deny remote access", "Negar acesso remoto"),
        ("Use IP Whitelisting", "Utilizar lista de IPs confiáveis"),
        ("Network", "Rede"),
        ("Enable RDP", "Habilitar RDP"),
        ("Pin menubar", "Fixar barra de menu"),
        ("Unpin menubar", "Desafixar barra de menu"),
        ("Recording", "Gravando"),
        ("Directory", "Diretório"),
        ("Automatically record incoming sessions", "Gravar automaticamente sessões de entrada"),
        ("Change", "Alterar"),
        ("Start session recording", "Iniciar gravação da sessão"),
        ("Stop session recording", "Parar gravação da sessão"),
        ("Enable Recording Session", "Habilitar gravação da sessão"),
        ("Allow recording session", "Permitir gravação da sessão"),
        ("Enable LAN Discovery", "Habilitar descoberta da LAN"),
        ("Deny LAN Discovery", "Negar descoberta da LAN"),
        ("Write a message", "Escrever uma mensagem"),
        ("Prompt", "Prompt de comando"),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Desconectado"),
        ("Other", "Outro"),
        ("Confirm before closing multiple tabs", "Confirmar antes de fechar múltiplas abas"),
        ("Keyboard Settings", "Configurações de teclado"),
        ("Full Access", "Acesso completo"),
        ("Screen Share", "Compartilhamento de tela"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requer Ubuntu 21.04 ou versão superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requer uma versão superior da distribuição linux. Por favor, tente o desktop X11 ou mude seu sistema operacional."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Por favor, selecione a tela a ser compartilhada (operar no lado do parceiro)."),
        ("Show RustDesk", "Exibir RustDesk"),
        ("This PC", "Este PC"),
        ("or", "ou"),
        ("Continue with", "Continuar com"),
        ("Elevate", "Elevar"),
        ("Zoom cursor", "Aumentar cursor"),
        ("Accept sessions via password", "Aceitar sessões via senha"),
        ("Accept sessions via click", "Aceitar sessões via clique"),
        ("Accept sessions via both", "Aceitar sessões de ambos os modos"),
        ("Please wait for the remote side to accept your session request...", "Por favor aguarde enquanto o cliente remoto aceita seu pedido de sessão..."),
        ("One-time Password", "Senha de uso único"),
        ("Use one-time password", "Usar senha de uso único"),
        ("One-time password length", "Comprimento da senha de uso único"),
        ("Request access to your device", "Solicitar acesso ao seu dispositivo"),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("idd_driver_tip", ""),
        ("confirm_idd_driver_tip", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
    ].iter().cloned().collect();
}
