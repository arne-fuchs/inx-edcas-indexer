create table if not exists system
(
    timestamp      bigint,
    name           varchar NOT NULL,
    address        bigint  NOT NULL,
    body_count     integer,
    non_body_count integer,
    population     integer,
    allegiance     varchar,
    economy        varchar,
    second_economy varchar,
    government     varchar,
    security       varchar,
    faction        varchar,

    x              real,
    y              real,
    z              real,

    odyssey        boolean NOT NULL,

    UNIQUE (name, odyssey),
    UNIQUE (address, odyssey),
    primary key (address, odyssey)
);

create table if not exists system_faction
(
    timestamp      bigint,
    name           varchar,
    system_address bigint,
    faction_state  varchar,
    government     varchar,
    influence      real,
    allegiance     varchar,
    happiness      varchar,
    odyssey        boolean NOT NULL,

    primary key (name, system_address, odyssey),
    foreign key (system_address, odyssey) references system (address, odyssey)
);

create table if not exists faction_active_state
(
    timestamp      bigint,
    system_address bigint,
    faction        varchar,
    state          varchar,
    odyssey        boolean NOT NULL,

    foreign key (faction, system_address, odyssey) references system_faction (name, system_address, odyssey),
    primary key (faction, system_address, odyssey)
);

create table if not exists faction_recovering_state
(
    timestamp      bigint,
    system_address bigint,
    faction        varchar,
    state          varchar,
    trend          real,
    odyssey        boolean NOT NULL,

    foreign key (faction, system_address, odyssey) references system_faction (name, system_address, odyssey),
    primary key (faction, system_address, odyssey)
);

create table if not exists conflict
(
    timestamp      bigint,
    war_type       varchar,
    status         varchar,
    faction1       varchar,
    faction2       varchar,
    system_address bigint,
    odyssey        boolean NOT NULL,

    foreign key (system_address, odyssey) references system (address, odyssey),
    primary key (system_address, faction1, faction2, odyssey)
);

create table if not exists body
(
    timestamp                bigint,
    system_address           bigint,
    id                       integer NOT NULL,
    name                     varchar NOT NULL,

    ascending_node           real,
    axial_tilt               real,
    atmosphere               varchar,
    distance_from_arrival_ls real,
    eccentricity             real,
    landable                 boolean,
    mass_em                  real,
    mean_anomaly             real,
    orbital_inclination      real,
    orbital_period           real,
    periapsis                real,
    class                    varchar,
    radius                   real,
    rotation_period          real,
    semi_major_axis          real,
    surface_gravity          real,
    surface_pressure         real,
    surface_temperature      real,
    terraform_state          varchar,
    tidal_lock               boolean,
    volcanism                varchar,
    discovered               boolean,
    mapped                   boolean,
    odyssey        boolean NOT NULL,

unique (id,system_address),
    primary key (id, system_address, odyssey),
    foreign key (system_address, odyssey) references system (address, odyssey)
);

create table if not exists body_composition
(
    timestamp      bigint,
    body_id        integer,
    system_address bigint,
    name           varchar,
    percentage     real,
    odyssey        boolean NOT NULL,

    foreign key (body_id, system_address, odyssey) references body (id, system_address, odyssey),
    primary key (body_id, system_address, odyssey)
);

create table if not exists body_material
(
    timestamp      bigint,
    body_id        integer,
    system_address bigint,
    name           varchar,
    percentage     real,
    odyssey        boolean NOT NULL,


    foreign key (body_id, system_address, odyssey) references body (id, system_address, odyssey),
    primary key (body_id, system_address, name, odyssey)
);

create table if not exists atmosphere_composition
(
    timestamp      bigint,
    system_address bigint,
    body_id        integer,
    name           varchar,
    percent        real,
    odyssey        boolean NOT NULL,

    primary key (system_address, body_id, odyssey),
    foreign key (system_address, body_id, odyssey) references body (system_address, id, odyssey)
);

create table if not exists star
(
    timestamp                bigint,
    system_address           bigint,
    name                     varchar UNIQUE NOT NULL,
    id                       integer,
    absolute_magnitude       real,
    age_my                   integer,
    ascending_node           real,
    axial_tilt               real,
    distance_from_arrival_ls real,
    eccentricity             real,
    luminosity               varchar,
    mean_anomaly             real,
    orbital_inclination      real,
    orbital_period           real,
    periapsis                real,
    radius                   real,
    rotation_period          real,
    semi_major_axis          real,
    type                     varchar,
    stellar_mass             real,
    subclass                 integer,
    surface_temperature      real,
    discovered               boolean,
    mapped                   boolean,
    odyssey                  boolean NOT NULL,

    primary key (system_address, id, odyssey),
    foreign key (system_address, odyssey) references system (address, odyssey)
);

create table if not exists ring
(
    timestamp      bigint,
    system_address bigint,
    body_id        integer,
    name           varchar,
    inner_rad      real,
    outer_rad      real,
    mass_mt        real,
    class          varchar,
    odyssey        boolean NOT NULL,

    primary key (system_address, body_id, odyssey),
    foreign key (system_address, body_id, odyssey) references body (system_address, id, odyssey)
);


create table if not exists body_signal
(
    timestamp      bigint,
    system_address bigint,
    body_id        integer,
    count          integer,
    type           varchar,
    odyssey        boolean NOT NULL,

    primary key (system_address, body_id, odyssey, type),
    foreign key (system_address, body_id, odyssey) references body (system_address, id, odyssey)
);

create table if not exists station
(
    timestamp   bigint,
    name        varchar,
    market_id   bigint NOT NULL,
    system_name varchar,
    odyssey     boolean NOT NULL,

    primary key (market_id, odyssey),
    foreign key (system_name, odyssey) references system (name, odyssey)
);

create table if not exists ship
(
    timestamp bigint,
    market_id bigint,
    ship      varchar,
    odyssey   boolean NOT NULL,

    primary key (market_id, ship, odyssey),
    foreign key (market_id, odyssey) references station (market_id, odyssey)
);

create table if not exists module
(
    timestamp bigint,
    market_id bigint,
    name      varchar,
    odyssey   boolean NOT NULL,

    primary key (market_id, name, odyssey),
    foreign key (market_id, odyssey) references station (market_id, odyssey)
);

create table if not exists commodity
(
    timestamp      bigint,
    market_id      bigint,
    name           varchar,
    buy_price      integer,
    sell_price     integer,
    mean_price     integer,
    demand_bracket integer,
    stock          integer,
    stock_bracket  integer,
    odyssey        boolean NOT NULL,

    primary key (market_id, name, odyssey),
    foreign key (market_id, odyssey) references station (market_id, odyssey)
);

create table if not exists parent
(
    system_address bigint NOT NULL,
    body_id        integer NOT NULL,
    parent_type    varchar,
    parent_id      integer,
    odyssey        boolean NOT NULL,

    primary key (system_address,body_id,parent_id, odyssey),
    foreign key (system_address,body_id, odyssey) references body(system_address,id, odyssey)
);

create table if not exists pid
(
    pkey varchar primary key
)