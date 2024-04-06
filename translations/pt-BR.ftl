# Comandos
ping = ping
     .description = Faz o bot responder com Pong!

help = ajuda
    .description = Lista de comandos
    .command = comando
    .command-description = Nome do comando

database = database
    .description = Executa uma query de testes no banco de dados

infractions = infractions
    .description = Infrações
    .add = add
    .add-description = Adicionar nova infração à tabela de infrações
    .add-id = id
    .add-id-description = ID da infração
    .add-severity = severidade
    .add-severity-description = Severidade da infração
    .add-severity-Low = Baixa severidade
    .add-severity-Mid = Média severidade
    .add-severity-High = Alta severidade
    .add-punishment = punição
    .add-punishment-description = Punição da infração
    .add-punishment-Strike = Advertência
    .add-punishment-Timeout = Mutar o usuário
    .add-punishment-Ban = Banimento
    .add-punishment-Kick = Expulsar
    .add-duration = duração
    .add-duration-description = Duração do timeout
    .list = list
    .list-description = Tabela de infrações
    .remove = remove
    .remove-description = Remover infração da tabela de infrações
    .remove-id = id
    .remove-id-description = ID da infração a ser removida da tabela
    .user = user
    .user-description = Mostra as infrações que um usuário específico cometeu
    .user-member = membro
    .user-member-description = Membro do servidor

kick = kick
     .description = Expulsar usuários
     .users = users
     .users-description = Usuários que serão expulsos
     .reason = reason
     .reason-description = Motivo para expulsar os usuários

timeout = timeout
     .description = Mutar usuários por tempo determinado
     .users = users
     .users-description = Usuários que serão mutados
     .time = time
     .time-description = Tempo em números
     .unit = unit
     .unit-description = Unidade de tempo
     .unit-Seconds = Segundos (time * 1)
     .unit-Minutes = Minutos (time * 60)
     .unit-Hours = Horas (time * 60 * 60)
     .unit-Days = Dias (time * 60 * 60 * 24)

untimeout = untimeout
     .description = Desmutar usuários
     .users = users
     .users-description = Usuários que serão desmutados

ban = ban
     .description = Banir usuários do servidor
     .users = users
     .users-description = Usuários que serão banidos
     .reason = reason
     .reason-description = Motivo para banir os usuários

unban = unban
     .description = Desbanir usuários do servidor
     .users = users
     .users-description = Usuários que serão desbanidos

strike = strike
     .description = Dar strike em usuários
     .users = users
     .users-description = Usuários que levarão strike
     .reason = reason
     .reason-description = Motivo para dar strike nos usuários

punish = punish
     .description = Punir usuários
     .id = id
     .id-description = ID da infração
     .users = users
     .users-description = Usuários que serão punidos
     .message = message
     .message-description = Motivo para punir os usuários

tag = tag
    .description = Comando de tag
    .add = add
    .add-description = Criar uma nova tag
    .add-name = name
    .add-name-description = Nome da tag
    .add-content = content
    .add-content-description = Conteúdo da tag
    .edit = edit
    .edit-description = Editar uma tag sua
    .edit-name = name
    .edit-name-description = Nome da tag
    .edit-content = content
    .edit-content-description = Novo conteúdo da tag
    .see = see
    .see-description = Mostra o conteúdo da tag
    .see-name = name
    .see-name-description = Nome da tag
    .list = list
    .list-description = Mostra as tags do servidor
    .user = user
    .user-description = Mostra as tags do usuário
    .user-user = user
    .user-user-description = Usuário
    .remove = remove
    .remove-description = Deleta uma tag sua
    .remove-name = name
    .remove-name-description = Nome da tag

emoji = emoji
    .description = Comando de emoji
    .see = see
    .see-description = Mostra o emoji especificado
    .see-emoji = emoji
    .see-emoji-description = Emoji que você quer ver
    .add = add
    .add-description = Adiciona um novo emoji ao servidor
    .add-name = name
    .add-name-description = Nome do novo emoji
    .add-attachment = attachment
    .add-attachment-description = Imagem do novo emoji
    .list = list
    .list-description = Mostra os emojis do servidor
    .remove = remove
    .remove-description = Remove um emoji do servidor
    .remove-emoji = emoji
    .remove-emoji-description = Emoji que será removido

# Respostas
Pong = Pong! :ping_pong: