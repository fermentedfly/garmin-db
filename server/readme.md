# Postgres Setup

https://stackoverflow.com/questions/10431426/trying-to-get-postgres-setup-in-my-environment-but-cant-seem-to-get-permissions

Answer 2

```bash
sudo mkdir /var/lib/postgres
sudo chmod 775 /var/lib/postgres
sudo chown postgres /var/lib/postgres

sudo -i -u postgres

[postgres]$ initdb --locale $LANG -E UTF8 -D '/var/lib/postgres/data'
[postgres]$ exit

sudo systemctl start postgresql.service

sudo -i -u postgres

[postgres]$  createuser --interactive # use same username as login name
exit

createdb garminDB
```

# Diesel Setup

Export url to database created above+
```bash
echo DATABASE_URL=postgres://postgres:password@localhost/garminDB > .env
```