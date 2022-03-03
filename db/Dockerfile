FROM mysql:latest

RUN touch /var/run/mysqld/mysqld.sock \
    && touch /var/run/mysqld/mysqld.pid \
    && chown -R mysql:mysql /var/run/mysqld/mysqld.sock \
    && chown -R mysql:mysql /var/run/mysqld/mysqld.sock \
    && chown -R mysql:mysql /var/run/mysqld \
    && chmod -R 644 /var/run/mysqld/mysqld.sock

ADD init.sql /docker-entrypoint-initdb.d