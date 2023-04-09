module Tokenizers
  module FromPretrained
    # for user agent
    TOKENIZERS_VERSION = "0.13.3"

    # use Ruby for downloads
    # this avoids the need to vendor OpenSSL on Linux
    # and reduces the extension size by about half
    def from_pretrained(identifier, revision: "main", auth_token: nil)
      require "cgi"
      require "digest"
      require "fileutils"
      require "json"
      require "open-uri"

      cache_dir = ensure_cache_dir

      # string options are headers
      options = {
        open_timeout: 3,
        read_timeout: 30,
        "User-Agent" => "tokenizers/#{TOKENIZERS_VERSION}; bindings/Ruby; version/#{VERSION}"
      }
      if auth_token
        options["Authorization"] = "Bearer #{auth_token}"
      end

      url = "https://huggingface.co/%s/resolve/%s/tokenizer.json" % [identifier, revision].map { |v| CGI.escape(v) }

      path =
        begin
          cached_path(cache_dir, url, options)
        rescue OpenURI::HTTPError
          raise Error, "Model \"#{identifier}\" on the Hub doesn't have a tokenizer"
        end

      from_file(path)
    end

    private

    # use same storage format as Rust version
    # https://github.com/epwalsh/rust-cached-path
    def cached_path(cache_dir, url, options)
      fsum = Digest::SHA256.hexdigest(url)
      meta_paths = Dir[File.join(cache_dir, "#{fsum}.*.meta")]
      meta = meta_paths.map { |f| JSON.parse(File.read(f)) }.max_by { |m| m["creation_time"] }
      etag = meta["etag"] if meta

      if etag
        esum = Digest::SHA256.hexdigest(etag)
        resource_path = File.join(cache_dir, "#{fsum}.#{esum}")
        options["If-None-Match"] = etag if File.exist?(resource_path)
      end

      options[:content_length_proc] = -> (_) { puts "Downloading..." }

      tempfile =
        begin
          URI.parse(url).open(options)
        rescue OpenURI::HTTPError => e
          if e.message == "304 Not Modified"
            return resource_path
          else
            raise e
          end
        end

      etag = tempfile.meta["etag"]
      esum = Digest::SHA256.hexdigest(etag)
      resource_path = File.join(cache_dir, "#{fsum}.#{esum}")
      meta_path = "#{resource_path}.meta"

      meta = {
        resource: url,
        resource_path: resource_path,
        meta_path: meta_path,
        etag: etag,
        expires: nil,
        creation_time: Time.now.to_f
      }

      File.write("#{resource_path}.lock", "")
      File.open(resource_path, "wb") { |f| IO.copy_stream(tempfile, f) }
      File.write(meta_path, JSON.generate(meta))

      resource_path
    end

    def cache_dir
      if ENV["TOKENIZERS_CACHE"]
        ENV["TOKENIZERS_CACHE"]
      else
        # use same directory as Rust version
        # https://docs.rs/dirs/latest/dirs/fn.cache_dir.html
        dir =
          if Gem.win_platform?
            ENV.fetch("LOCALAPPDATA")
          elsif mac?
            File.join(ENV.fetch("HOME"), "Library", "Caches")
          else
            ENV["XDG_CACHE_HOME"] || File.join(ENV.fetch("HOME"), ".cache")
          end

        File.join(dir, "huggingface", "tokenizers")
      end
    end

    def ensure_cache_dir
      dir = cache_dir
      FileUtils.mkdir_p(dir)
      dir
    end

    def mac?
      RbConfig::CONFIG["host_os"] =~ /darwin/i
    end
  end
end
