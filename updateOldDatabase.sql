alter table parents RENAME TO parent;
alter table body_signals RENAME TO body_signal;
alter table body_materials RENAME TO body_material;
alter table conflicts RENAME TO conflict;

alter table parent drop constraint parent_pkey;
alter table parent add constraint parent_pkey PRIMARY KEY (system_address,body_id,parent_id, odyssey);

drop table faction_recovering_state;
drop table faction_active_state;
drop table system_faction;

alter table body drop constraint body_system_address_id_key;
alter table star drop constraint star_name_key;
alter table body_composition drop constraint body_composition_body_id_system_address_name_key;

alter table atmosphere_composition rename percent to percentage;
alter table atmosphere_composition drop constraint atmosphere_composition_pkey;
alter table atmosphere_composition add constraint atmosphere_composition_pkey PRIMARY KEY (system_address, body_id, odyssey);