pub use sea_orm_migration::prelude::*;

mod m_000001_scan_batch;
mod m_000002_ip_main;
mod m_000003_ip_privacy;
mod m_000004_ip_location;
mod m_000005_ip_connection;
mod m_000006_ip_network_details;
mod m_000007_ip_security_flags;
mod m_000008_ip_organization;
mod m_000009_ip_contact_details;
mod m_000010_ip_abuse_contact;
mod m_000011_ip_flag;
mod m_000012_ip_hosting_details;
mod m_000013_ip_metadata;
mod m_000014_ip_address;
mod m_000015_ip_port;
mod m_000016_ip_port_state;
mod m_000017_ip_service;
mod m_000018_ip_stats;
mod m_000019_ip_times;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m_000001_scan_batch::Migration)]
    }
}
