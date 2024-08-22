-- Notifications types

-- banner: a 3:1 image with optional title and target URL
-- post: classic news post with title, content, and optional 1:1 image
-- popup: a modal dialog with title, content, and optional 1:1 image

-- Notes

-- banner and posts are shown together in a single carousel. use popup for special announcements
-- banner and posts share the same priority queue, where banners take precedence

-- ads can be scheduled to appear at a specific time and disappear at a specific time
-- if begin_at is null, the ad is immediately active
-- if end_at is null, the ad is active indefinitely

CREATE TABLE IF NOT EXISTS notifications (
    id serial PRIMARY KEY,
    is_active boolean NOT NULL DEFAULT FALSE,
    ad_title varchar(1024),
    ad_type varchar(8) NOT NULL,
    priority integer NOT NULL DEFAULT 0, -- order, from lowest to highest
    image_url varchar(2084), -- banner images should have a 3:1 aspect ratio, null image will be interpreted as drafts
    target_url varchar(2084), -- URL to redirect to when the ad is clicked
    -- scheduling
    begin_at timestamptz,
    end_at timestamptz,
    -- When type is post or popup, the following fields are required
    ad_content varchar(4096),
    -- Metadata
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CHECK (priority >= 0),
    CHECK (begin_at IS NULL OR end_at IS NULL OR begin_at < end_at),
    CONSTRAINT valid_ad_type CHECK (ad_type IN ('banner', 'post', 'popup')),
    CONSTRAINT banner_has_image CHECK ((ad_type = 'banner' AND image_url IS NOT NULL) OR ad_type != 'banner'),
    CONSTRAINT post_has_title CHECK ((ad_type = 'post' AND ad_title IS NOT NULL) OR ad_type != 'post'),
    CONSTRAINT post_has_content CHECK ((ad_type = 'post' AND ad_content IS NOT NULL) OR ad_type != 'post'),
    CONSTRAINT popup_has_title CHECK ((ad_type = 'popup' AND ad_title IS NOT NULL) OR ad_type != 'popup'),
    CONSTRAINT popup_has_content CHECK ((ad_type = 'popup' AND ad_content IS NOT NULL) OR ad_type != 'popup')
);

CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON notifications FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();
