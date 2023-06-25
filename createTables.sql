PRAGMA foreign_keys = ON;

create table if not exists body (
    timestamp text,
    system_name text,
    id integer,
    name text,

    ascending_node real,
    axial_tilt real,
    atmosphere text,
    distance_from_arrival_ls real,
    eccentricity real,
    landable integer,
    mass_em real,
    mean_anomaly real,
    orbital_inclination real,
    orbital_period real,
    periapsis real,
    class text,
    radius real,
    rotation_period real,
    semi_major_axis real,
    surface_gravity real,
    surface_pressure real,
    surface_temperature real,
    terraform_state text,
    tidal_lock integer,
    volcanism text,
    discovered integer,
    mapped integer,

    primary key (id,system_name),
    foreign key (system_name) references system(name)
);

create table if not exists body_composition (
    timestamp text,
    body_id integer,
    system_name text,
    name text,
    percentage real,

    foreign key (body_id,system_name) references body(id,system_name),
    primary key (body_id,system_name,name)
);

create table if not exists body_materials (
    timestamp text,
    body_id integer,
    system_name text,
    name text,
    percentage real,

    foreign key (body_id,system_name) references body(id,system_name),
    primary key (body_id,system_name,name)
);

create table if not exists atmosphere_composition (
    timestamp text,
    system_name text ,
    body_id integer,
    name text,
    percent real,

    primary key (system_name,body_id),
    foreign key (system_name,body_id) references body(system_name,id)
);

create table if not exists star (
    timestamp text,
    system_address text,
    name text,
    id integer,
    absolute_magnitued real,
    age_my integer,
    ascending_node real,
    axial_tilt real,
    distance_from_arrival_ls real,
    eccentricity real,
    luminosity text,
    mean_anomaly real,
    orbital_inclination real,
    orbital_period real,
    periapsis real,
    radius real,
    rotation_period real,
    semi_major_axis real,
    type text,
    stellar_mass real,
    subclass integer,
    surface_temperature real,
    discovered integer,
    mapped integer,

    primary key (system_address,id),
    foreign key (system_address) references system(address)
);

create table if not exists ring (
    timestamp text,
    system_address text,
    name text,
    inner_rad real,
    outer_rad real,
    mass_mt real,
    class text,

    primary key (system_address,name),
    foreign key (system_address) references system(address)
);


create table if not exists body_signals (
    timestamp text,
    system_address text,
    body_id integer,
    count integer,
    type text,

    primary key (system_address,body_id),
    foreign key (system_address,body_id) references body(system_name,id)
);

create table if not exists system (
    timestamp text,
    name text,
    address text,
    body_count integer,
    non_body_count integer,
    population integer,
    allegiance text,
    economy text,
    second_economy text,
    government text,
    security text,
    faction text,

    x real,
    y real,
    z real,

    primary key (address)
);

create table if not exists system_faction (
    timestamp text,
    name text,
    system_address text,
    faction_state text,
    government text,
    influence real,
    allegiance text,
    happiness text,

    primary key (name,system_address),
    foreign key (system_address) references system(address)
);

create table if not exists faction_active_state (
    timestamp text,
    faction text,
    state text,

    foreign key (faction) references system_faction(name),
    primary key (faction,state)
);

create table if not exists faction_recovering_state (
    timestamp text,
    faction text,
    state text,
    trend real,

    foreign key (faction) references system_faction(name),
    primary key (faction,state)
);

create table if not exists conflicts (
    timestamp text,
    war_type text,
    status text,
    faction1 text,
    faction2 text,
    system text,

    foreign key (faction1,faction2) references system_faction(name,name),
    primary key (system,faction1,faction2)
);

create table if not exists station (
    timestamp text,
    station_name text,
    market_id text,
    system_name text,

    primary key (market_id),
    foreign key (system_name) references system(name)
);

create table if not exists commodity (
  timestamp text,
  market_id text,
  name text,
  buy_price integer,
  sell_price integer,
  mean_price integer,
  demand_bracket integer,
  stock integer,
  stock_bracket integer,

  primary key (market_id,name),
  foreign key (market_id) references station(market_id)
);
