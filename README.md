# Watchdog

Aplicativo para monitoramento de arquivos e notificação de modificações com base em templates 

## Início Rápido

```bash
git clone https://github.com/realfabecker/watchdog project
cd project
cargo run
```

## Instalando

Na maioria dos sistemas Unix a instalação do watchdog pode ser realizada com o seguinte comando:

```bash
curl -so- https://raw.githubusercontent.com/realfabecker/watchdog/master/install.sh | bash
```

Ao término da instalação será solicitado para configuração do aplicativo em seu $PATH de acesso;

```bash
echo export PATH=$PATH:$HOME/.watchdog/bin >> ~/.bin_bash
```

## Configurando

o watchdog é orientado a um arquivo de configuração para seu uso base

```yaml
watches:
  file_pattern: ^docs

message:
  template: |
    Pull Request Proposed changes:
    
    *User*: {{actor}}
    *Pull Request*: {{title}}
    *Link*: {{link}}
    
    {{changes}}

notify:
  url: ${WD_NOTIFY_URL}
  token: ${WD_NOTIFY_TOKEN}
  body: |
    {
      "topic": "${WD_NOTIFY_TOPIC}",
      "subject": "Notify Subject",
      "message": "{{message}}"
    }
```

O arquivo anterior representa o seguinte cenário:

* Para arquivos alterados com nome prefixo ^docs.
* Gerar template mensagem notificação campos-chave substituídos.
* E encaminhar mensagem gerado por requisição http template.

## Usando

A invocação do aplicativo se dá por linha de comando por preenchimento de seus atributos obrigatórios:

```bash
watchdog --actor ${{ github.actor }} \
    --from ${{ github.event.before }} \
    --to ${{ github.event.after }} \
    --link ${{ github.event.pull_request.html_url }} \
    --title "Sample Notifiable Pull Request" \
    --config ./docs/config/wd-pr.yml
```

A chamada anterior considera ambiente cd github onde as informaçẽos são contidas no objeto github.

## Change log

Verifique o [CHANGELOG](CHANGELOG.md) para informações sobre alterações recentes.

## Dependências

* [OpenSSL](https://www.openssl.org/)

## Contribuições

Refira-se ao guia de [contribuições](./docs/CONTRIBUTING.md) para detalhes de como contribuir para o projeto.

## Versionamento

O projeto utilizada [SemVer](https://semver.org/) para o versionamento. Para todas as versões disponíveis verifique as
[tags nesse repositório][project-link].

## Licença

Este projeto considera a licença MIT. Verifique a [Licença](./docs/LICENSE.md) para mais informações.

[project-link]: https://github.com/realfabecker/watchdog