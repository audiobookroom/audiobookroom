use std log info
use std log warning
info "app starting"
# get the env var PUID and PGID
let PUID = do {
    if ($env.PUID? | is-empty) {
        0
    } else {
        $env.PUID | into int
    }
}

let PGID = do {
    if ($env.PGID? | is-empty) {   
        0
    } else {
        $env.PGID | into int
    }
}


let db = do {
    if ($env.DB? | is-empty) {
        "mysql://root:audiobookroom@mysql:3306/audiobookroom"
    } else {
        $env.DB
    }
}

info $"PUID: ($PUID), PGID: ($PGID), DB: ($db)"

$env.DATABASE_URL = $db
$env.LEPTOS_SITE_ROOT = "./site"
$env.LEPTOS_SITE_ADDR = "0.0.0.0:3000"

mut db_inited = false
while not $db_inited {
    let result = do {
        mariadb-admin ping -h mysql --silent
    } | complete
    if $result.exit_code == 0 {
        info "database is ready"
        $db_inited = true
    } else {
        warning "database is not ready, waiting for 1 second..."
        sleep 1sec
    }
}
let result = do {
    sh -c $"useradd -u ($PUID) -o -m -d /home/audiobookroom audiobookroom && chown -R ($PUID):($PGID) /app && chmod a+x ./audiobookroom"
} | complete
info ($result | to text)
if ("./fetchbook/inited_db.lock" | path exists ) {
    info "inited_db.lock exists"
} else {
    info "inited_db.lock does not exist"
    info "running seaorm migration"
    mariadb -h mysql -u root -paudiobookroom -e "DROP DATABASE IF EXISTS audiobookroom; create database audiobookroom;"
    ./migration --database-url $db fresh
    
    echo "1" | save -f "./fetchbook/inited_db.lock"
}


echo "\n" | save -f .env
# echo $"DATABASE_URL=($db)\n" | save --append .env
# echo "LEPTOS_SITE_ROOT=./site\n" | save --append .env
# echo "LEPTOS_SITE_ADDR=0.0.0.0:3000\n" | save --append .env
open .env | info $in
info "running audiobookroom"
su -c './audiobookroom' audiobookroom