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
mod m_000013_ip_os;
mod m_000014_ip_service;
mod m_000015_ip_service_extra;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_000001_scan_batch::Migration),
            Box::new(m_000002_ip_main::Migration),
            Box::new(m_000003_ip_privacy::Migration),
            Box::new(m_000004_ip_location::Migration),
            Box::new(m_000005_ip_connection::Migration),
            Box::new(m_000006_ip_network_details::Migration),
            Box::new(m_000007_ip_security_flags::Migration),
            Box::new(m_000008_ip_organization::Migration),
            Box::new(m_000009_ip_contact_details::Migration),
            Box::new(m_000010_ip_abuse_contact::Migration),
            Box::new(m_000011_ip_flag::Migration),
            Box::new(m_000012_ip_hosting_details::Migration),
            Box::new(m_000013_ip_os::Migration),
            Box::new(m_000014_ip_service::Migration),
            Box::new(m_000015_ip_service_extra::Migration),
        ]
    }
}
