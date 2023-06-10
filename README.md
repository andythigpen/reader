# Development

## Migrations

```
DATABASE_URL=sqlite://reader.db sea-orm-cli migrate status
```

## Entities

```
sea-orm-cli generate entity -u sqlite://reader.db --with-serde both -o server/entity/src/ -l
```
