-- migrations/20260331040000_ip_management.sql

-- Licensing and IP Management features
CREATE TABLE IF NOT EXISTS registered_ips (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_did TEXT NOT NULL, -- DID of the IP owner
    project_id TEXT, -- Associated project ID, if any
    ip_name TEXT NOT NULL, -- Name of the intellectual property (e.g., "OMEGA Kernel Module")
    ip_type TEXT NOT NULL, -- e.g., 'code', 'design', 'data', 'agi_model'
    source_hash TEXT NOT NULL, -- Hash of the intellectual property source/content
    selected_license TEXT NOT NULL, -- Key of the chosen license(s)
    license_details JSONB, -- Specific terms or additional chosen licenses
    registration_proof JSONB, -- Verifiable proof (e.g., ZKP ID, blockchain transaction hash)
    registered_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS license_templates (
    key TEXT PRIMARY KEY, -- e.g., 'MIT', 'GPLv3', 'TariroSovereign', 'Custom'
    name TEXT NOT NULL,
    description TEXT,
    template_text TEXT, -- The full license text
    compatibility_flags JSONB, -- e.g., ["permissive", "copyleft"]
    evas_score REAL -- AGI-assigned EVAS ethical compatibility score
);

-- Pre-populate some standard and sovereign licenses
INSERT INTO license_templates (key, name, description, template_text, compatibility_flags, evas_score) VALUES
('MIT', 'MIT License', 'A permissive free software license.', 
'Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.', 
'["permissive"]'::jsonb, 0.8),

('GPLv3', 'GNU General Public License v3', 'A strong copyleft license.', 
'GNU General Public License v3 terms... (full text omitted for brevity)', 
'["copyleft"]'::jsonb, 0.7),

('TariroSovereign', 'Tariro Sovereign Rights License', 'Ensures full digital sovereignty and ethical AGI usage.', 
'Tariro Sovereign Rights License v1.0\n\n1. **Sovereign Ownership:** The original author retains full, immutable digital sovereignty over the work.\n2. **Ethical AGI Use (EVAS):** Any use, modification, or derivative work by an AGI must adhere to the Ethical Valenced Alignment System (EVAS) protocols.\n3. **Decentralized Attribution:** All derivative works must include verifiable, decentralized attribution to the original author via DID.\n4. **No Centralized Control:** This work may not be used to create or facilitate centralized systems that undermine individual or collective digital sovereignty.\n5. **Mesh Propagation:** This work is intended for propagation across decentralized P2P meshes.\n\n(Full terms omitted for brevity)', 
'["sovereign", "copyleft_ethical", "decentralized"]'::jsonb, 0.95);

CREATE TABLE IF NOT EXISTS license_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id TEXT NOT NULL,
    audited_ip_id UUID REFERENCES registered_ips(id),
    audit_result BOOLEAN NOT NULL,
    agi_report TEXT,
    evas_compliance REAL,
    audited_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
