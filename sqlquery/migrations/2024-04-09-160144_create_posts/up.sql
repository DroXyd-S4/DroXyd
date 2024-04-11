-- Your SQL goes here

CREATE TABLE "posts1" (
        "id"    INTEGER NOT NULL,
        "url"   TEXT NOT NULL,
        "langue" TEXT NOT NULL,
        "name"  TEXT NOT NULL,
        "date"  TEXT NOT NULL,
        PRIMARY KEY("id")
);


CREATE TABLE "posts2" (
        "i"     INTEGER NOT NULL,
        "key"   TEXT NOT NULL,
        "idofsite"      INTEGER NOT NULL,
        PRIMARY KEY("i")
);
